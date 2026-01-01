//! UI modules - immediate mode, stateless rendering with mouse support

use macroquad::prelude::*;

// Import toolkit utilities
pub use macroquad_toolkit::input::{is_mouse_over, was_clicked, was_pressed};

/// Draw a button and return true if clicked
///
/// Note: Frontier uses a different parameter order (text first) than other games
pub fn button(text: &str, x: f32, y: f32, w: f32, h: f32) -> bool {
    let style = macroquad_toolkit::ui::ButtonStyle {
        normal: Color::from_rgba(40, 50, 50, 255),
        hovered: Color::from_rgba(60, 80, 60, 255),
        pressed: Color::from_rgba(80, 100, 80, 255),
        border: GRAY,
        text_color: LIGHTGRAY,
    };

    // Frontier uses on_release (was_clicked) behavior
    macroquad_toolkit::ui::button_on_release(x, y, w, h, text, &style)
}

/// Draw a button with custom colors
#[allow(dead_code)]
pub fn button_colored(text: &str, x: f32, y: f32, w: f32, h: f32, base_color: Color) -> bool {
    let hovered = is_mouse_over(x, y, w, h);
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);

    // Lighten/darken based on state
    let bg_color = if pressed {
        Color::from_rgba(
            (base_color.r * 255.0 * 0.7) as u8,
            (base_color.g * 255.0 * 0.7) as u8,
            (base_color.b * 255.0 * 0.7) as u8,
            255,
        )
    } else if hovered {
        Color::from_rgba(
            ((base_color.r * 255.0 * 1.2).min(255.0)) as u8,
            ((base_color.g * 255.0 * 1.2).min(255.0)) as u8,
            ((base_color.b * 255.0 * 1.2).min(255.0)) as u8,
            255,
        )
    } else {
        base_color
    };
    draw_rectangle(x, y, w, h, bg_color);

    // Border
    let border_color = if hovered { WHITE } else { GRAY };
    draw_rectangle_lines(x, y, w, h, 2.0, border_color);

    // Text (centered)
    let text_size = 18.0;
    let text_width = text.len() as f32 * 9.0;
    let text_x = x + (w - text_width) / 2.0;
    let text_y = y + (h + text_size) / 2.0 - 2.0;
    draw_text(text, text_x, text_y, text_size, WHITE);

    was_clicked(x, y, w, h)
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
