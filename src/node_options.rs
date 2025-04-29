use serde::Deserialize;
use ts_rs::TS;

use crate::{color::Color, node::NodeType};

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct NodeOptions {
    #[ts(optional)]
    pub node_type: Option<NodeType>,
    #[ts(optional)]
    pub is_static: Option<bool>,
    #[ts(optional)]
    pub is_stretchy: Option<bool>,
    #[ts(optional)]
    pub is_smart_stretch: Option<bool>,
    #[ts(optional)]
    pub do_not_apply_smart_stretch: Option<bool>,
    #[ts(optional)]
    pub use_segment_color: Option<bool>,
    #[ts(optional)]
    pub use_circle_outline: Option<bool>,
    #[ts(optional)]
    pub circle_is_hollow: Option<bool>,
    #[ts(optional)]
    pub use_gradient: Option<bool>,
    #[ts(optional)]
    pub reverse_gradient: Option<bool>,
    #[ts(optional)]
    pub gradient_mode: Option<i16>,
    #[ts(optional)]
    pub use_segment_scale: Option<bool>,
    #[ts(optional)]
    pub local_x: Option<f32>,
    #[ts(optional)]
    pub local_y: Option<f32>,
    #[ts(optional)]
    pub scale: Option<f32>,
    #[ts(optional)]
    pub default_length: Option<f32>,
    #[ts(optional)]
    pub length: Option<f32>,
    #[ts(optional)]
    pub default_thickness: Option<i32>,
    #[ts(optional)]
    pub thickness: Option<i32>,
    #[ts(optional)]
    pub segment_curve_radius_and_default_curve_radius: Option<i32>,
    #[ts(optional)]
    pub curve_circulization: Option<bool>,
    #[ts(optional)]
    pub segment_curve_polyfill_precision: Option<i16>,
    #[ts(optional)]
    pub half_arc: Option<bool>,
    #[ts(optional)]
    pub right_triangle_direction: Option<i16>,
    #[ts(optional)]
    pub triangle_upside_down: Option<bool>,
    #[ts(optional)]
    pub trapezoid_top_thickness_ratio: Option<f32>,
    #[ts(optional)]
    pub num_polygon_vertices: Option<i16>,
    #[ts(optional)]
    pub default_local_angle: Option<f32>,
    #[ts(optional)]
    pub local_angle: Option<f32>,
    #[ts(optional)]
    pub default_angle: Option<f32>,
    #[ts(optional)]
    pub color: Option<Color>,
    #[ts(optional)]
    pub gradient_color: Option<Color>,
    #[ts(optional)]
    pub circle_outline_color: Option<Color>,
}

impl Default for NodeOptions {
    fn default() -> Self {
        NodeOptions::from(sticknodes_rs::NodeOptions::default())
    }
}

impl From<sticknodes_rs::NodeOptions> for NodeOptions {
    fn from(options: sticknodes_rs::NodeOptions) -> Self {
        NodeOptions {
            node_type: Some(NodeType::from(options.node_type)),
            is_static: Some(options.is_static),
            is_stretchy: Some(options.is_stretchy),
            is_smart_stretch: Some(options.is_smart_stretch),
            do_not_apply_smart_stretch: Some(options.do_not_apply_smart_stretch),
            use_segment_color: Some(options.use_segment_color),
            use_circle_outline: Some(options.use_circle_outline),
            circle_is_hollow: Some(options.circle_is_hollow),
            use_gradient: Some(options.use_gradient),
            reverse_gradient: Some(options.reverse_gradient),
            gradient_mode: Some(options.gradient_mode),
            use_segment_scale: Some(options.use_segment_scale),
            local_x: Some(options.local_x),
            local_y: Some(options.local_y),
            scale: Some(options.scale),
            default_length: Some(options.default_length),
            length: Some(options.length),
            default_thickness: Some(options.default_thickness),
            thickness: Some(options.thickness),
            segment_curve_radius_and_default_curve_radius: Some(
                options.segment_curve_radius_and_default_curve_radius,
            ),
            curve_circulization: Some(options.curve_circulization),
            segment_curve_polyfill_precision: Some(options.segment_curve_polyfill_precision),
            half_arc: Some(options.half_arc),
            right_triangle_direction: Some(options.right_triangle_direction),
            triangle_upside_down: Some(options.triangle_upside_down),
            trapezoid_top_thickness_ratio: Some(options.trapezoid_top_thickness_ratio),
            num_polygon_vertices: Some(options.num_polygon_vertices),
            default_local_angle: Some(options.default_local_angle),
            local_angle: Some(options.local_angle),
            default_angle: Some(options.default_angle),
            color: Some(Color::from(options.color)),
            gradient_color: Some(Color::from(options.gradient_color)),
            circle_outline_color: Some(Color::from(options.circle_outline_color)),
        }
    }
}

impl From<NodeOptions> for sticknodes_rs::NodeOptions {
    fn from(options: NodeOptions) -> Self {
        let defaults = sticknodes_rs::NodeOptions::default();

        sticknodes_rs::NodeOptions {
            node_type: match options.node_type {
                Some(node_type) => sticknodes_rs::NodeType::from(node_type),
                None => defaults.node_type,
            },
            is_static: options.is_static.unwrap_or(defaults.is_static),
            is_stretchy: options.is_stretchy.unwrap_or(defaults.is_stretchy),
            is_smart_stretch: options
                .is_smart_stretch
                .unwrap_or(defaults.is_smart_stretch),
            do_not_apply_smart_stretch: options
                .do_not_apply_smart_stretch
                .unwrap_or(defaults.do_not_apply_smart_stretch),
            use_segment_color: options
                .use_segment_color
                .unwrap_or(defaults.use_segment_color),
            use_circle_outline: options
                .use_circle_outline
                .unwrap_or(defaults.use_circle_outline),
            circle_is_hollow: options
                .circle_is_hollow
                .unwrap_or(defaults.circle_is_hollow),
            use_gradient: options.use_gradient.unwrap_or(defaults.use_gradient),
            reverse_gradient: options
                .reverse_gradient
                .unwrap_or(defaults.reverse_gradient),
            gradient_mode: options.gradient_mode.unwrap_or(defaults.gradient_mode),
            use_segment_scale: options
                .use_segment_scale
                .unwrap_or(defaults.use_segment_scale),
            local_x: options.local_x.unwrap_or(defaults.local_x),
            local_y: options.local_y.unwrap_or(defaults.local_y),
            scale: options.scale.unwrap_or(defaults.scale),
            default_length: options.default_length.unwrap_or(defaults.default_length),
            length: options.length.unwrap_or(defaults.length),
            default_thickness: options
                .default_thickness
                .unwrap_or(defaults.default_thickness),
            thickness: options.thickness.unwrap_or(defaults.thickness),
            segment_curve_radius_and_default_curve_radius: options
                .segment_curve_radius_and_default_curve_radius
                .unwrap_or(defaults.segment_curve_radius_and_default_curve_radius),
            curve_circulization: options
                .curve_circulization
                .unwrap_or(defaults.curve_circulization),
            segment_curve_polyfill_precision: options
                .segment_curve_polyfill_precision
                .unwrap_or(defaults.segment_curve_polyfill_precision),
            half_arc: options.half_arc.unwrap_or(defaults.half_arc),
            right_triangle_direction: options
                .right_triangle_direction
                .unwrap_or(defaults.right_triangle_direction),
            triangle_upside_down: options
                .triangle_upside_down
                .unwrap_or(defaults.triangle_upside_down),
            trapezoid_top_thickness_ratio: options
                .trapezoid_top_thickness_ratio
                .unwrap_or(defaults.trapezoid_top_thickness_ratio),
            num_polygon_vertices: options
                .num_polygon_vertices
                .unwrap_or(defaults.num_polygon_vertices),
            default_local_angle: options
                .default_local_angle
                .unwrap_or(defaults.default_local_angle),
            local_angle: options.local_angle.unwrap_or(defaults.local_angle),
            default_angle: options.default_angle.unwrap_or(defaults.default_angle),
            color: match options.color {
                Some(color) => sticknodes_rs::Color::from(color),
                None => defaults.color,
            },
            gradient_color: match options.gradient_color {
                Some(color) => sticknodes_rs::Color::from(color),
                None => defaults.gradient_color,
            },
            circle_outline_color: match options.circle_outline_color {
                Some(color) => sticknodes_rs::Color::from(color),
                None => defaults.circle_outline_color,
            },
        }
    }
}
