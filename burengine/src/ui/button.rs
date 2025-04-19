use crate::{graphics::Graphics, types::{Color, Rect}};

use super::{Draw, DrawingContext, Widget};

pub struct Button {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    text: String,
    is_pressed: bool,
}

impl Widget for Button {}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text: text.to_string(),
            is_pressed: false,
        }
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.is_pressed = pressed;
    }
    
    #[allow(dead_code)]
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

impl Draw for Button {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let scale = context.scale;
        let x = self.x + context.parent_x;
        let y = self.y + context.parent_y;
        
        // Create base rect and scale it
        let base_rect = Rect::new(x, y, self.width, self.height);
        let scaled_rect = base_rect.scale(scale);

        let background_color = Color::from_array(context.config.widgets.button.defaults.background_color);
        let text_color = Color::from_array(context.config.widgets.button.defaults.text_color);
        
        // Draw background
        let bg_color = if self.is_pressed {
            // Darken color when pressed
            let r = (background_color.r as f32 * 0.8) as u8;
            let g = (background_color.g as f32 * 0.8) as u8;
            let b = (background_color.b as f32 * 0.8) as u8;
            Color::RGBA(r, g, b, background_color.a)
        } else {
            background_color
        };
        
        // Draw the button background
        graphics.draw_rect(scaled_rect, bg_color);
        graphics.draw_rect_outline(scaled_rect, text_color, 1);
        
        // Draw text centered on button
        let text_scale = scale * 1.0;
        graphics.draw_text_centered(
            &self.text,
            scaled_rect.x + (scaled_rect.width / 2) as i32,
            scaled_rect.y + (scaled_rect.height / 2) as i32,
            text_scale,
            text_color,
        );
    }
} 