use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};
static NODE_COUNTER: AtomicUsize = AtomicUsize::new(1);
use sticknodes_rs::DrawOrderIndex;
use ts_rs::TS;

use std::{cell::RefCell, rc::Rc};

use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::{
    color::Color, macros::wasm_node_primitive_getters_setters, node_options::NodeOptions,
    stickfigure::Stickfigure,
};

#[wasm_bindgen]
pub struct Node {
    id: usize,
    inner: Rc<RefCell<sticknodes_rs::Node>>,
    pub(crate) stickfigure: Rc<Stickfigure>,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> usize {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn stickfigure(&self) -> Stickfigure {
        self.stickfigure.deref().clone()
    }

    #[wasm_bindgen(getter)]
    pub fn draw_index(&self) -> usize {
        self.get_draw_index().0 as usize
    }

    #[wasm_bindgen(getter)]
    pub fn node_type(&self) -> NodeType {
        NodeType::from(self.inner.borrow().node_type.clone())
    }

    #[wasm_bindgen(setter)]
    pub fn set_node_type(&self, node_type: NodeType) {
        self.inner.borrow_mut().node_type = sticknodes_rs::NodeType::from(node_type);
    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> Color {
        Color::from(self.inner.borrow().color)
    }

    #[wasm_bindgen(setter)]
    pub fn set_color(&self, color: Color) {
        self.inner.borrow_mut().color = sticknodes_rs::Color::from(color);
    }

    #[wasm_bindgen(getter)]
    pub fn gradient_color(&self) -> Color {
        Color::from(self.inner.borrow().gradient_color)
    }

    #[wasm_bindgen(setter)]
    pub fn set_gradient_color(&self, color: Color) {
        self.inner.borrow_mut().gradient_color = sticknodes_rs::Color::from(color);
    }

    #[wasm_bindgen(getter)]
    pub fn circle_outline_color(&self) -> Color {
        Color::from(self.inner.borrow().circle_outline_color)
    }

    #[wasm_bindgen(setter)]
    pub fn set_circle_outline_color(&self, color: Color) {
        self.inner.borrow_mut().circle_outline_color = sticknodes_rs::Color::from(color);
    }

    /// Set the draw index of this node to a new draw index.
    /// Will increment all indices that are >= the desired draw index if the desired draw index is already occupied.
    pub fn set_draw_index(&self, draw_index: usize) -> Result<(), JsError> {
        let current_node_draw_index = self.get_draw_index();
        self.stickfigure
            .change_draw_index(current_node_draw_index, DrawOrderIndex(draw_index as i32))?;
        Ok(())
    }

    /// Get the index of every node that is a direct child of this node.
    pub fn get_child_indices(&self) -> Result<Vec<usize>, JsError> {
        let current_node_draw_index = self.get_draw_index();
        Ok(self
            .stickfigure
            .get_child_node_draw_indices(current_node_draw_index)
            .iter()
            .map(|draw_index| draw_index.0 as usize)
            .collect())
    }

    /// Get the reference of every node that is a direct child of this node.
    pub fn get_child_nodes(&self) -> Result<Vec<Node>, JsError> {
        Ok(self.stickfigure.get_nodes(self.get_child_indices()?)?)
    }

    /// Get the index of every node that has the same parent as this node.
    pub fn get_sibling_indices(&self) -> Result<Vec<usize>, JsError> {
        let current_node_draw_index = self.get_draw_index();
        Ok(self
            .stickfigure
            .get_sibling_node_draw_indices(current_node_draw_index)
            .iter()
            .map(|draw_index| draw_index.0 as usize)
            .collect())
    }

    /// Get the reference of every node that has the same parent as this node.
    pub fn get_sibling_nodes(&self) -> Result<Vec<Node>, JsError> {
        Ok(self.stickfigure.get_nodes(self.get_sibling_indices()?)?)
    }

    /// Get the index of every node that is a child of this node, recursively.
    /// Includes child nodes of child nodes of child nodes and so on.
    /// Use `get_child_indices()` to get only direct children of this node.
    /// Use `get_descendant_nodes()` to get the reference of descendant nodes instead.
    pub fn get_descendant_indices(&self) -> Result<Vec<usize>, JsError> {
        let current_node_draw_index = self.get_draw_index();
        Ok(self
            .stickfigure
            .get_children_recursive(current_node_draw_index)
            .iter()
            .map(|draw_index| draw_index.0 as usize)
            .collect())
    }

    /// Get the reference of every node that is a child of this node, recursively.
    /// Includes child nodes of child nodes of child nodes and so on.
    /// Use `get_child_nodes()` to get only direct children of this node.
    /// Use `get_descendant_indices()` to get the index of descendant nodes instead.
    pub fn get_descendant_nodes(&self) -> Result<Vec<Node>, JsError> {
        Ok(self.stickfigure.get_nodes(self.get_descendant_indices()?)?)
    }

    /// Get the index of every node that is a parent of this node, recursively.
    /// Includes parent node of parent node of parent node and so on.
    /// Use `get_parent_index()` to get only the direct parent of this node.
    /// Use `get_ancestor_nodes()` to get the reference of ancestor nodes instead.
    pub fn get_ancestor_indices(&self) -> Result<Vec<usize>, JsError> {
        let current_node_draw_index = self.get_draw_index();
        Ok(self
            .stickfigure
            .get_parents_recursive(current_node_draw_index)
            .iter()
            .map(|draw_index| draw_index.0 as usize)
            .collect())
    }

    /// Get the reference of every node that is a parent of this node, recursively.
    /// Includes parent node of parent node of parent node and so on.
    /// Use `get_parent_node()` to get only the direct parent of this node.
    /// Use `get_ancestor_indices()` to get the index of ancestor nodes instead.
    pub fn get_ancestor_nodes(&self) -> Result<Vec<Node>, JsError> {
        Ok(self.stickfigure.get_nodes(self.get_ancestor_indices()?)?)
    }

    /// Get the index of the direct parent of this node.
    /// Use `get_parent_node()` to get the reference of the parent node instead.
    pub fn get_parent_index(&self) -> Result<usize, JsError> {
        let current_node_draw_index = self.get_draw_index();
        Ok(self
            .stickfigure
            .get_parent_node_draw_index(current_node_draw_index)
            .0 as usize)
    }

    /// Get the reference of the direct parent of this node.
    /// Use `get_parent_index()` to get the index of the parent node instead.
    pub fn get_parent_node(&self) -> Result<Node, JsError> {
        Ok(self.stickfigure.get_node(self.get_parent_index()?)?)
    }

    pub fn get_node_options(&self) -> Result<JsValue, JsError> {
        Ok(serde_wasm_bindgen::to_value(
            &self.inner.borrow().to_options(),
        )?)
    }

    /// Add a new node as a sibling of this node (a child of this node's parent).
    pub fn add_sibling(&self, options: JsValue) -> Result<Node, JsError> {
        let opts: NodeOptions = if !options.is_undefined() {
            serde_wasm_bindgen::from_value(options)
                .map_err(|e| JsError::new(&format!("Invalid node options: {}", e)))?
        } else {
            NodeOptions::from(sticknodes_rs::NodeOptions::default())
        };

        let final_options = sticknodes_rs::NodeOptions::from(opts);

        let current_node_draw_index = self.get_draw_index();

        let parent_node_draw_index = {
            self.stickfigure
                .get_parent_node_draw_index(current_node_draw_index)
        };

        self.stickfigure
            .add_node(final_options, parent_node_draw_index)
    }

    /// Add a new node as a child of this node.
    pub fn add_child(&self, options: JsValue) -> Result<Node, JsError> {
        let opts: NodeOptions = if !options.is_undefined() {
            serde_wasm_bindgen::from_value(options)
                .map_err(|e| JsError::new(&format!("Invalid node options: {}", e)))?
        } else {
            NodeOptions::from(sticknodes_rs::NodeOptions::default())
        };

        let final_options = sticknodes_rs::NodeOptions::from(opts);

        let current_node_draw_index = self.get_draw_index();

        self.stickfigure
            .add_node(final_options, current_node_draw_index)
    }

    /// Delete this current node.
    pub fn delete(&self) -> Result<(), JsError> {
        let current_node_draw_index = self.get_draw_index();
        Ok(self.stickfigure.remove_node(current_node_draw_index)?)
    }
}

// See macros.rs
// This macro generates wasm getters and setters for all the Node primitive types listed within this macro invocation
wasm_node_primitive_getters_setters!(
    (is_static, set_is_static, bool),
    (is_stretchy, set_is_stretchy, bool),
    (is_smart_stretch, set_is_smart_stretch, bool),
    (
        do_not_apply_smart_stretch,
        set_do_not_apply_smart_stretch,
        bool
    ),
    (use_segment_color, set_use_segment_color, bool),
    (use_circle_outline, set_use_circle_outline, bool),
    (circle_is_hollow, set_circle_is_hollow, bool),
    (use_gradient, set_use_gradient, bool),
    (reverse_gradient, set_reverse_gradient, bool),
    (gradient_mode, set_gradient_mode, i16),
    (use_segment_scale, set_use_segment_scale, bool),
    (local_x, set_local_x, f32),
    (local_y, set_local_y, f32),
    (scale, set_scale, f32),
    (default_length, set_default_length, f32),
    (length, set_length, f32),
    (default_thickness, set_default_thickness, i32),
    (thickness, set_thickness, i32),
    (
        segment_curve_radius_and_default_curve_radius,
        set_segment_curve_radius_and_default_curve_radius,
        i32
    ),
    (curve_circulization, set_curve_circulization, bool),
    (
        segment_curve_polyfill_precision,
        set_segment_curve_polyfill_precision,
        i16
    ),
    (half_arc, set_half_arc, bool),
    (right_triangle_direction, set_right_triangle_direction, i16),
    (triangle_upside_down, set_triangle_upside_down, bool),
    (
        trapezoid_top_thickness_ratio,
        set_trapezoid_top_thickness_ratio,
        f32
    ),
    (num_polygon_vertices, set_num_polygon_vertices, i16),
    (default_local_angle, set_default_local_angle, f32),
    (local_angle, set_local_angle, f32),
    (default_angle, set_default_angle, f32),
);

impl Node {
    pub(crate) fn new(
        rc_node: Rc<RefCell<sticknodes_rs::Node>>,
        stickfigure: Rc<Stickfigure>,
    ) -> Node {
        let id = NODE_COUNTER.fetch_add(1, Ordering::Relaxed);
        Node {
            id,
            inner: rc_node,
            stickfigure,
        }
    }

    pub(crate) fn get_draw_index(&self) -> DrawOrderIndex {
        self.inner.borrow().get_draw_order_index()
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq, Deserialize, TS)]
pub enum NodeType {
    RootNode = -1,
    RoundedSegment = 0,
    Segment = 1,
    Circle = 2,
    Triangle = 3,
    FilledCircle = 4,
    Ellipse = 5,
    Trapezoid = 6,
    Polygon = 7,
}

impl From<sticknodes_rs::NodeType> for NodeType {
    fn from(node_type: sticknodes_rs::NodeType) -> Self {
        match node_type {
            sticknodes_rs::NodeType::RootNode => NodeType::RootNode,
            sticknodes_rs::NodeType::RoundedSegment => NodeType::RoundedSegment,
            sticknodes_rs::NodeType::Segment => NodeType::Segment,
            sticknodes_rs::NodeType::Circle => NodeType::Circle,
            sticknodes_rs::NodeType::Triangle => NodeType::Triangle,
            sticknodes_rs::NodeType::FilledCircle => NodeType::FilledCircle,
            sticknodes_rs::NodeType::Ellipse => NodeType::Ellipse,
            sticknodes_rs::NodeType::Trapezoid => NodeType::Trapezoid,
            sticknodes_rs::NodeType::Polygon => NodeType::Polygon,
        }
    }
}

impl From<NodeType> for sticknodes_rs::NodeType {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::RootNode => sticknodes_rs::NodeType::RootNode,
            NodeType::RoundedSegment => sticknodes_rs::NodeType::RoundedSegment,
            NodeType::Segment => sticknodes_rs::NodeType::Segment,
            NodeType::Circle => sticknodes_rs::NodeType::Circle,
            NodeType::Triangle => sticknodes_rs::NodeType::Triangle,
            NodeType::FilledCircle => sticknodes_rs::NodeType::FilledCircle,
            NodeType::Ellipse => sticknodes_rs::NodeType::Ellipse,
            NodeType::Trapezoid => sticknodes_rs::NodeType::Trapezoid,
            NodeType::Polygon => sticknodes_rs::NodeType::Polygon,
        }
    }
}
