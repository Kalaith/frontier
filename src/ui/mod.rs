//! UI modules - immediate mode, stateless rendering with mouse support

use macroquad::prelude::*;

// Import toolkit utilities
pub use macroquad_toolkit::input::{is_mouse_over, was_clicked, was_pressed};

/// Draw a button and return true if clicked
///
/// Note: Frontier uses a different parameter order (text first) than other games
pub fn button(text: &str, x: f32, y: f32, w: f32, h: f32) -> bool {
    // Frontier uses a specific style
    let style = macroquad_toolkit::ui::ButtonStyle {
        normal: Color::from_rgba(40, 50, 50, 255),
        hovered: Color::from_rgba(60, 80, 60, 255),
        pressed: Color::from_rgba(80, 100, 80, 255),
        border: GRAY,
        text_color: LIGHTGRAY,
        disabled: Color::new(0.1, 0.1, 0.1, 1.0),
    };

    // Frontier uses on_release behavior
    macroquad_toolkit::ui::button_on_release(x, y, w, h, text, &style)
}

/// Draw a button with custom colors
#[allow(dead_code)]
pub fn button_colored(text: &str, x: f32, y: f32, w: f32, h: f32, base_color: Color) -> bool {
    // Use the toolkit helper but respecting frontier's argument order
    macroquad_toolkit::ui::colored_button(x, y, w, h, text, base_color)
}

/// A clickable card/panel - returns true if clicked, also handles hover highlighting
#[allow(dead_code)]
pub struct ClickableRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[allow(dead_code)]
impl ClickableRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn is_hovered(&self) -> bool {
        is_mouse_over(self.x, self.y, self.w, self.h)
    }

    pub fn was_clicked(&self) -> bool {
        was_clicked(self.x, self.y, self.w, self.h)
    }

    pub fn was_pressed(&self) -> bool {
        was_pressed(self.x, self.y, self.w, self.h)
    }
}
