use std::sync::atomic::{AtomicUsize, Ordering};
static STICKFIGURE_COUNTER: AtomicUsize = AtomicUsize::new(1);

use std::{cell::RefCell, rc::Rc};

use sticknodes_rs::{DrawOrderIndex, StickfigureError};
use wasm_bindgen::prelude::*;

use crate::color::Color;
use crate::node::Node;
use crate::polyfill::Polyfill;
use crate::polyfill_options::PolyfillOptions;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Stickfigure {
    id: usize,
    inner: Rc<RefCell<sticknodes_rs::Stickfigure>>
}

/// This is test documentation
#[wasm_bindgen]
impl Stickfigure {
    /// This is also test documentation changekjhasjfdh
    #[wasm_bindgen(constructor)]
    pub fn new() -> Stickfigure {
        let id = STICKFIGURE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let stickfigure = sticknodes_rs::Stickfigure::new();
        let inner = Rc::new(RefCell::new(stickfigure));
        Stickfigure { id, inner }
    }

    pub fn from_bytes(mut bytes: &[u8]) -> Result<Stickfigure, JsError> {
        let id = STICKFIGURE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let stickfigure = sticknodes_rs::Stickfigure::from_bytes(&mut bytes).or_else(|e| Err(e))?;
        let inner = Rc::new(RefCell::new(stickfigure));
        Ok(Stickfigure { id, inner })
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, JsError> {
        let stickfigure = self.inner.borrow();
        Ok(stickfigure.to_bytes().or_else(|e| Err(e))?)
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> i32 {
        self.inner.borrow().version
    }

    pub fn set_version(&self, version_code: usize) -> Result<(), JsError> {
        self.inner.borrow_mut().version = version_code as i32;
        Ok(())
    }

    #[wasm_bindgen(getter)]
    pub fn scale(&self) -> f32 {
        self.inner.borrow().scale
    }

    #[wasm_bindgen(setter)]
    pub fn set_scale(&self, scale: f32) {
        self.inner.borrow_mut().scale = scale;
    }

    #[wasm_bindgen(getter)]
    pub fn build(&self) -> i32 {
        self.inner.borrow().build
    }

    #[wasm_bindgen(setter)]
    pub fn set_build(&self, build: i32) {
        self.inner.borrow_mut().build = build;
    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> Color {
        let snrs_color = self.inner.borrow().color;
        Color {red: snrs_color.red, green: snrs_color.green, blue: snrs_color.blue, alpha: snrs_color.alpha }
    }

    #[wasm_bindgen(setter)]
    pub fn set_color(&self, color: Color) {
        let snrs_color = sticknodes_rs::Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha };
        self.inner.borrow_mut().color = snrs_color;
    }

    pub fn to_jsobject(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.borrow().to_serializable()).expect("Something went wrong with serializing Stickfigure.")
    }

    pub fn add_polyfill(&self, options: JsValue) -> Result<Polyfill, JsError> {
        let opts: PolyfillOptions = if !options.is_undefined() {
            serde_wasm_bindgen::from_value(options).map_err(|e| JsError::new(&format!("Invalid polyfill options: {}", e)))?
        } else {
            PolyfillOptions::from(sticknodes_rs::PolyfillOptions::default())
        };

        let final_options = sticknodes_rs::PolyfillOptions::from(opts);


        let rc_poly: Rc<RefCell<sticknodes_rs::Polyfill>> =  {
            let mut inner: std::cell::RefMut<'_, sticknodes_rs::Stickfigure> = self.inner.borrow_mut();

            let polyfill: sticknodes_rs::Polyfill = sticknodes_rs::Polyfill::from_options(final_options, self.inner.borrow().clone())?;

            let index: usize = inner.add_polyfill(polyfill).0 as usize;

            inner.polyfills[index].clone()
        };

        // get rc wrapped wasm-bindgen stickfigure to give to wasm-bindgen polyfill
        let rc_stickfigure: Rc<Stickfigure> = Rc::new(self.clone());

        let poly = Polyfill::new(rc_poly, rc_stickfigure);

        Ok(poly)
    }

    pub fn get_polyfill(&self, anchor_draw_index: usize) -> Result<Polyfill, JsError> {
        let option = self.inner.borrow().get_polyfill(DrawOrderIndex(anchor_draw_index as i32));

        match option {
            Some(polyfill) => Ok(Polyfill::new(Rc::clone(&polyfill), Rc::new(self.clone()))),
            None => Err(JsError::new(format!("Polyfill not found. Searched for index {}.", anchor_draw_index).as_str()))
        }
    }

    pub fn get_all_polyfills(&self) -> Vec<Polyfill> {
        self.inner.borrow().polyfills.iter().map(|poly| Polyfill::new(Rc::clone(poly), Rc::new(self.clone()))).collect()
    }

    pub fn get_node(&self, draw_index: usize) -> Result<Node, JsError> {
        if let Some(rc_node) = self.inner.borrow().get_node(DrawOrderIndex(draw_index as i32)) {
            let rc_stickfigure = Rc::new(Stickfigure {
                id: self.id,
                inner: Rc::clone(&self.inner),
            });
            let node = Node::new(rc_node.clone(), rc_stickfigure);
            Ok(node)
        } else {
            Err(JsError::new(format!("Node not found. Searched for index {}.", draw_index).as_str()))
        }
    }

    pub fn get_nodes(&self, draw_indices: Vec<usize>) -> Result<Vec<Node>, JsError> {
        draw_indices.iter().map(|draw_index| self.get_node(*draw_index)).collect()
    }

    pub fn get_all_node_indices(&self) -> Result<Vec<usize>, JsError> {
        let indices: Vec<usize> = self.inner.borrow().get_all_node_indices().iter().map(|draw_index| draw_index.0 as usize).collect();
        Ok(indices)
    }

    pub fn get_all_nodes(&self) -> Result<Vec<Node>, JsError> {
        let all_node_indices = self.get_all_node_indices()?;
        self.get_nodes(all_node_indices)
    }

    // pub fn get_nodes? maybe take a closure if supported?


}

impl Stickfigure {
    pub(crate) fn add_node(&self, options: sticknodes_rs::NodeOptions, parent_draw_index: DrawOrderIndex) -> Result<Node, JsError> {
        let draw_index = {
            self.inner.borrow_mut().add_node(sticknodes_rs::Node::from_options(options), parent_draw_index)?
        };
        if let Some(rc_node) = self.inner.borrow().get_node(draw_index) {
            let rc_stickfigure = Rc::new(Stickfigure {
                id: self.id,
                inner: Rc::clone(&self.inner),
            });
            let node = Node::new(rc_node.clone(), rc_stickfigure);
            Ok(node)
        } else {
            Err(JsError::new("INTERAL ERROR: Parent node not found. Searched for index {}. This was not expected to happen. Most likely an error with sticknodes-js, not you."))
        }
    }

    pub(crate) fn get_parent_node_draw_index(&self, draw_index: DrawOrderIndex) -> DrawOrderIndex {
        self.inner.borrow().get_parent(draw_index)
    }
    
    pub(crate) fn get_child_node_draw_indices(&self, draw_index: DrawOrderIndex) -> Vec<DrawOrderIndex> {
        self.inner.borrow().get_children(draw_index)
    }

    pub(crate) fn get_sibling_node_draw_indices(&self, draw_index: DrawOrderIndex) -> Vec<DrawOrderIndex> {
        self.inner.borrow().get_siblings(draw_index)
    }

    pub(crate) fn change_draw_index(&self, draw_index: DrawOrderIndex, new_draw_index: DrawOrderIndex) -> Result<(), StickfigureError> {
        self.inner.borrow_mut().change_draw_index(draw_index, new_draw_index)
    }

    pub(crate) fn get_children_recursive(&self, draw_index: DrawOrderIndex) -> Vec<DrawOrderIndex> {
        self.inner.borrow().get_children_recursive(draw_index)
    }

    pub(crate) fn get_parents_recursive(&self, draw_index: DrawOrderIndex) -> Vec<DrawOrderIndex> {
        self.inner.borrow().get_parents_recursive(draw_index)
    }

    pub(crate) fn remove_node(&self, draw_index: DrawOrderIndex) -> Result<(), StickfigureError> {
        Ok(self.inner.borrow_mut().remove_node(draw_index)?)
    }

    // polyfills

    pub fn set_anchor_node_draw_index(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_index: DrawOrderIndex) -> Result<(), JsError> {
        Ok(polyfill_rc.borrow_mut().set_anchor_node_draw_index(draw_index, self.inner.borrow().clone())?)

    }

    pub(crate) fn set_attached_node_draw_indices(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_indices: Vec<DrawOrderIndex>) -> Result<(), JsError> {
        Ok(polyfill_rc.borrow_mut().set_attached_node_draw_indices(draw_indices, self.inner.borrow().clone())?) // cloning stickfigure to check indices should be fine, but keep an eye on it
    }

    pub(crate) fn insert_attached_node_draw_indices_after(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_indices: Vec<DrawOrderIndex>, draw_index: DrawOrderIndex) -> Result<(), JsError> {
        Ok(polyfill_rc.borrow_mut().insert_attached_node_draw_indices_after(draw_indices, draw_index, self.inner.borrow().clone())?) // cloning stickfigure to check indices should be fine, but keep an eye on it
    }

    pub(crate) fn insert_attached_node_draw_indices_before(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_indices: Vec<DrawOrderIndex>, draw_index: DrawOrderIndex) -> Result<(), JsError> {
        Ok(polyfill_rc.borrow_mut().insert_attached_node_draw_indices_before(draw_indices, draw_index, self.inner.borrow().clone())?) // cloning stickfigure to check indices should be fine, but keep an eye on it

    }

    pub(crate) fn remove_attached_node_draw_indices(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_indices: Vec<DrawOrderIndex>) -> Result<(), JsError> {
        Ok(polyfill_rc.borrow_mut().remove_attached_node_draw_indices(draw_indices, self.inner.borrow().clone())?) // cloning stickfigure to check indices should be fine, but keep an eye on it
    }

    pub(crate) fn try_set_attached_node_draw_indices(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_indices: Vec<DrawOrderIndex>) -> Vec<DrawOrderIndex> {
        polyfill_rc.borrow_mut().try_set_attached_node_draw_indices(draw_indices, self.inner.borrow().clone()) // cloning stickfigure to check indices should be fine, but keep an eye on it

    }

    // below methods are commented out because I don't know if they'd be worth existing since there would have to be some kind of special return for if the provided index to insert after/before is invalid.. which kind of defeats the point of the method. At that point just handle the error of the non try versions lol
    // pub(crate) fn try_insert_attached_node_draw_indices_after(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_indices: Vec<DrawOrderIndex>, draw_index: DrawOrderIndex) -> Vec<DrawOrderIndex> {
    //     // let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
    //     // Ok(polyfill_rc.borrow_mut().set_attached_node_draw_indices(draw_indices_mapped, self.inner.borrow().clone())?) // cloning stickfigure to check indices should be fine, but keep an eye on it
    // }

    // pub(crate) fn try_insert_attached_node_draw_indices_before(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_indices: Vec<DrawOrderIndex>, draw_index: DrawOrderIndex) -> Vec<DrawOrderIndex> {
    //     // let draw_indices_mapped = draw_indices.iter().map(|index| DrawOrderIndex(*index)).collect();
    //     // Ok(polyfill_rc.borrow_mut().set_attached_node_draw_indices(draw_indices_mapped, self.inner.borrow().clone())?) // cloning stickfigure to check indices should be fine, but keep an eye on it
    // }

    pub(crate) fn try_remove_attached_node_draw_indices(&self, polyfill_rc: Rc<RefCell<sticknodes_rs::Polyfill>>, draw_indices: Vec<DrawOrderIndex>) -> Vec<DrawOrderIndex> {
        polyfill_rc.borrow_mut().try_remove_attached_node_draw_indices(draw_indices, self.inner.borrow().clone()) // cloning stickfigure to check indices should be fine, but keep an eye on it

    }


}