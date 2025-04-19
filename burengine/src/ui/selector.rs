use crate::{graphics::Graphics, types::{Color, Rect}};

use super::{Draw, DrawingContext, Widget};

pub struct Selector {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    options: Vec<String>,
    selected_index: usize,
}

impl Widget for Selector {}

impl Selector {
    pub fn new(x: i32, y: i32, width: u32, height: u32, options: Vec<String>) -> Self {       
        Self {
            x,
            y,
            width,
            height,
            options,
            selected_index: 0,
        }
    }
    
    #[allow(dead_code)]
    pub fn set_selected_index(&mut self, index: usize) {
        if index < self.options.len() {
            self.selected_index = index;
        }
    }
    
    #[allow(dead_code)]
    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }
    
    pub fn get_selected_option(&self) -> Option<&str> {
        self.options.get(self.selected_index).map(|s| s.as_str())
    }
    
    #[allow(dead_code)]
    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else if !self.options.is_empty() {
            self.selected_index = self.options.len() - 1;
        }
    }
    
    #[allow(dead_code)]
    pub fn select_next(&mut self) {
        if !self.options.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.options.len();
        }
    }
}

impl Draw for Selector {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let scale = context.scale;
        let x = self.x + context.parent_x;
        let y = self.y + context.parent_y;
        let text_color = Color::from_array(context.config.widgets.selector.defaults.text_color);
        
        // Create base rect and scale it
        let base_rect = Rect::new(x, y, self.width, self.height);
        let scaled_rect = base_rect.scale(scale);
        
        // Draw the background using sprite
        let src = Rect::from_array(context.config.widgets.selector.sprites.background);
        graphics.draw_texture(&context.config.img_path, src, scaled_rect);
        
        // Draw selected option text
        if let Some(option) = self.get_selected_option() {
            graphics.draw_text_centered(
                option,
                scaled_rect.x + (scaled_rect.width / 2) as i32,
                scaled_rect.y + (scaled_rect.height / 2) as i32,
                scale,
                text_color,
            );
        }
        
        // Draw arrows
        let left_arrow = context.config.widgets.selector.sprites.left_arrow;
        let arrow_width_scaled = (left_arrow[2] as f32 * scale) as u32;
        let arrow_height_scaled = (left_arrow[3] as f32 * scale) as u32;
        let arrow_padding = (5.0 * scale) as i32;
        
        // Left arrow
        let left_arrow_x = scaled_rect.x + arrow_padding;
        let left_arrow_y = scaled_rect.y + (scaled_rect.height / 2) as i32 - (arrow_height_scaled / 2) as i32;
        
        let left_src = Rect::from_array(left_arrow);
        let left_dest = Rect::new(left_arrow_x, left_arrow_y, arrow_width_scaled, arrow_height_scaled);
        graphics.draw_texture(&context.config.img_path, left_src, left_dest);
        
        // Right arrow
        let right_arrow_x = scaled_rect.x + scaled_rect.width as i32 - arrow_width_scaled as i32 - arrow_padding;
        let right_arrow_y = scaled_rect.y + (scaled_rect.height / 2) as i32 - (arrow_height_scaled / 2) as i32;
        
        let right_arrow = context.config.widgets.selector.sprites.right_arrow;
        let right_src = Rect::from_array(right_arrow);
        let right_dest = Rect::new(right_arrow_x, right_arrow_y, arrow_width_scaled, arrow_height_scaled);
        graphics.draw_texture(&context.config.img_path, right_src, right_dest);
    }
} 