use crate::{graphics::Graphics, types::{Color, Rect}};

use super::{Draw, DrawingContext, Widget};

pub struct Label {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    text: String,
}

impl Widget for Label {}

impl Label {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text: text.to_string(),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Draw for Label {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let scale = context.scale;
        let x = self.x + context.parent_x;
        let y = self.y + context.parent_y;
        let text_color = Color::from_array(context.config.widgets.label.defaults.text_color);
        
        // Create base rect and scale it
        let base_rect = Rect::new(x, y, self.width, self.height);
        let scaled_rect = base_rect.scale(scale);
        
        // Draw text
        let text_scale = scale * 1.0;
        let text_x = scaled_rect.x;
        let text_y = scaled_rect.y + (scaled_rect.height / 2) as i32;
        
        graphics.draw_text(
            &self.text,
            text_x,
            text_y,
            text_scale,
            text_color,
        );
    }
} 