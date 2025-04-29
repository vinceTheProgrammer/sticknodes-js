use serde::Deserialize;
use sticknodes_rs::DrawOrderIndex;
use ts_rs::TS;

use crate::color::Color;

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct PolyfillOptions {
    #[ts(optional)]
    pub anchor_node_draw_index: Option<i32>,
    #[ts(optional)]
    pub color: Option<Color>,
    #[ts(optional)]
    pub use_polyfill_color: Option<bool>,
    #[ts(optional)]
    pub attached_node_draw_indices: Option<Vec<i32>>,
}

impl Default for PolyfillOptions {
    fn default() -> Self {
        PolyfillOptions::from(sticknodes_rs::PolyfillOptions::default())
    }
}

impl From<sticknodes_rs::PolyfillOptions> for PolyfillOptions {
    fn from(options: sticknodes_rs::PolyfillOptions) -> Self {
        PolyfillOptions {
            anchor_node_draw_index: Some(options.anchor_node_draw_index.0),
            color: Some(options.color.into()),
            use_polyfill_color: Some(options.use_polyfill_color),
            attached_node_draw_indices: Some(
                options
                    .attached_node_draw_indices
                    .iter()
                    .map(|i| i.0)
                    .collect(),
            ),
        }
    }
}

impl From<PolyfillOptions> for sticknodes_rs::PolyfillOptions {
    fn from(options: PolyfillOptions) -> Self {
        let defaults = sticknodes_rs::PolyfillOptions::default();

        let anchor_node_draw_index = options
            .anchor_node_draw_index
            .unwrap_or(defaults.anchor_node_draw_index.0);
        let color = options.color.unwrap_or(Color::from(defaults.color));
        let use_polyfill_color = options
            .use_polyfill_color
            .unwrap_or(defaults.use_polyfill_color);
        let attached_node_draw_indices = options.attached_node_draw_indices.unwrap_or(
            defaults
                .attached_node_draw_indices
                .iter()
                .map(|i| i.0)
                .collect(),
        );

        sticknodes_rs::PolyfillOptions {
            anchor_node_draw_index: DrawOrderIndex(anchor_node_draw_index),
            color: color.into(),
            use_polyfill_color,
            attached_node_draw_indices: attached_node_draw_indices
                .iter()
                .map(|i| DrawOrderIndex(*i))
                .collect(),
        }
    }
}
