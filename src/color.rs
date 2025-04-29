use serde::Deserialize;
use wasm_bindgen::prelude::*;
use ts_rs::TS;

#[wasm_bindgen]
#[derive(Clone, Copy, Deserialize, TS)]
pub struct Color {
    pub alpha: u8,
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl From<sticknodes_rs::Color> for Color {
    fn from(color: sticknodes_rs::Color) -> Self {
        Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
    }
}

impl From<Color> for sticknodes_rs::Color {
    fn from(color: Color) -> Self {
        sticknodes_rs::Color { alpha: color.alpha, blue: color.blue, green: color.green, red: color.red }
    }
}

#[wasm_bindgen]
impl Color {
    /// Creates a new color from RGBA values.
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let snrs_color = sticknodes_rs::Color::from_rgba(red, green, blue, alpha);
        Color {
            red: snrs_color.red,
            green: snrs_color.green,
            blue: snrs_color.blue,
            alpha: snrs_color.alpha,
        }
    }

    /// Creates a new color from RGB values (assumes alpha is 255).
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        let snrs_color = sticknodes_rs::Color::from_rgb(red, green, blue);
        Color {
            red: snrs_color.red,
            green: snrs_color.green,
            blue: snrs_color.blue,
            alpha: snrs_color.alpha,
        }
    }

    /// Creates a new color from a hex string.
    /// The string can be in the formats: "#RGB", "#RGBA", "#RRGGBB", "#RRGGBBAA", "RGB", "RGBA", "RRGGBB", or "RRGGBBAA" (case-insensitive). Any other format will fail.
    pub fn from_hex(hex: &str) -> Result<Self, JsError> {
        let snrs_color = sticknodes_rs::Color::from_hex(hex).or_else(|e| Err(e))?;
        Ok(Color {
            red: snrs_color.red,
            green: snrs_color.green,
            blue: snrs_color.blue,
            alpha: snrs_color.alpha,
        })
    }

    /// Converts the color to a hex string.
    pub fn to_hex(&self) -> String {
        let snrs_color = sticknodes_rs::Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha,
        };
        snrs_color.to_hex()
    }
}
