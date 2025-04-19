use burengine::{graphics::Graphics, types::{Color, Rect}};
use crate::ui::config::UI_CONFIG;

use super::{Draw, DrawingContext, Widget};

pub struct InputBox {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    text: String,
    placeholder: String,
    text_color: Color,
    placeholder_color: Color,
    background_color: Color,
    border_color: Color,
    cursor_position: usize,
    is_focused: bool,
    cursor_blink_timer: f32,
    show_cursor: bool,
}

impl Widget for InputBox {}

impl InputBox {
    pub fn new(x: i32, y: i32, width: u32, height: u32, placeholder: &str) -> Self {
        let config = &UI_CONFIG.widgets.input_box.defaults;
        
        Self {
            x,
            y,
            width,
            height,
            text: String::new(),
            placeholder: placeholder.to_string(),
            text_color: Color::from_array(config.text_color),
            placeholder_color: Color::from_array(config.placeholder_color),
            background_color: Color::from_array(config.background_color),
            border_color: Color::from_array(config.border_color),
            cursor_position: 0,
            is_focused: false,
            cursor_blink_timer: 0.0,
            show_cursor: false,
        }
    }
    
    #[allow(dead_code)]
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.cursor_position = self.text.len();
    }
    
    #[allow(dead_code)]
    pub fn get_text(&self) -> &str {
        &self.text
    }
    
    #[allow(dead_code)]
    pub fn set_focus(&mut self, focused: bool) {
        self.is_focused = focused;
        self.cursor_blink_timer = 0.0;
        self.show_cursor = focused;
    }
    
    #[allow(dead_code)]
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
    
    #[allow(dead_code)]
    pub fn update(&mut self, delta_time: f32) {
        if self.is_focused {
            self.cursor_blink_timer += delta_time;
            if self.cursor_blink_timer >= 0.5 {
                self.cursor_blink_timer = 0.0;
                self.show_cursor = !self.show_cursor;
            }
        }
    }
    
    // This would be connected to keyboard input events
    #[allow(dead_code)]
    pub fn handle_character(&mut self, c: char) {
        if self.is_focused {
            let mut chars = self.text.chars().collect::<Vec<_>>();
            chars.insert(self.cursor_position, c);
            self.text = chars.into_iter().collect();
            self.cursor_position += 1;
        }
    }
    
    #[allow(dead_code)]
    pub fn handle_backspace(&mut self) {
        if self.is_focused && self.cursor_position > 0 {
            let mut chars = self.text.chars().collect::<Vec<_>>();
            chars.remove(self.cursor_position - 1);
            self.text = chars.into_iter().collect();
            self.cursor_position -= 1;
        }
    }
}

impl Draw for InputBox {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let scale = context.scale;
        let x = self.x + context.parent_x;
        let y = self.y + context.parent_y;
        
        // Create base rect and scale it
        let base_rect = Rect::new(x, y, self.width, self.height);
        let scaled_rect = base_rect.scale(scale);
        
        // Draw background
        graphics.draw_rect(scaled_rect, self.background_color);
        
        // Draw border
        let border_thickness = 1;
        graphics.draw_rect_outline(scaled_rect, self.border_color, border_thickness);
        
        // Draw text or placeholder
        let text_padding = (5.0 * scale) as i32;
        let text_y = scaled_rect.y + text_padding;
        
        if self.text.is_empty() && !self.is_focused {
            graphics.draw_text(
                &self.placeholder,
                scaled_rect.x + text_padding,
                text_y,
                scale,
                self.placeholder_color,
            );
        } else {
            graphics.draw_text(
                &self.text,
                scaled_rect.x + text_padding,
                text_y,
                scale,
                self.text_color,
            );
            
            if self.is_focused && self.show_cursor {
                // TODO: Implement proper text width measurement when handling user input
                // For now, use a fixed character width approximation
                let approx_char_width = 8.0 * scale;
                let cursor_x = scaled_rect.x + text_padding + (self.cursor_position as f32 * approx_char_width) as i32;
                let cursor_rect = Rect::new(cursor_x, scaled_rect.y, 1, scaled_rect.height);
                graphics.draw_rect(cursor_rect, self.text_color);
            }
        }
    }
} 