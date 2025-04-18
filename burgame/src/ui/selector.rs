use burengine::{graphics::Graphics, types::{Color, Rect}};
use crate::ui::config::UI_CONFIG;

use super::{Draw, DrawingContext, Widget};

pub struct Selector {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    options: Vec<String>,
    selected_index: usize,
    background_color: Color,
    text_color: Color,
    arrow_color: Color,
}

impl Widget for Selector {}

impl Selector {
    pub fn new(x: i32, y: i32, width: u32, height: u32, options: Vec<String>) -> Self {
        let config = &UI_CONFIG.widgets.selector.defaults;
        
        Self {
            x,
            y,
            width,
            height,
            options,
            selected_index: 0,
            background_color: Color::from_array(config.background_color),
            text_color: Color::from_array(config.text_color),
            arrow_color: Color::from_array(config.arrow_color),
        }
    }
    
    pub fn set_selected_index(&mut self, index: usize) {
        if index < self.options.len() {
            self.selected_index = index;
        }
    }
    
    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }
    
    pub fn get_selected_option(&self) -> Option<&str> {
        self.options.get(self.selected_index).map(|s| s.as_str())
    }
    
    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else if !self.options.is_empty() {
            self.selected_index = self.options.len() - 1;
        }
    }
    
    pub fn select_next(&mut self) {
        if !self.options.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.options.len();
        }
    }
    
    pub fn add_option(&mut self, option: &str) {
        self.options.push(option.to_string());
    }
}

impl Draw for Selector {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let scale = context.scale;
        let x = self.x + context.parent_x;
        let y = self.y + context.parent_y;
        
        // Create base rect and scale it
        let base_rect = Rect::new(x, y, self.width, self.height);
        let scaled_rect = base_rect.scale(scale);
        
        // Draw the background using sprite
        let selector_sprite = &UI_CONFIG.widgets.selector.sprites.background;
        let src = Rect::from_array(*selector_sprite);
        graphics.draw_texture("resources/sprites/gui.png", src.to_sdl(), scaled_rect.to_sdl());
        
        // Draw selected option text
        if let Some(option) = self.get_selected_option() {
            graphics.draw_text(
                option,
                scaled_rect.x + (scaled_rect.width / 2) as i32,
                scaled_rect.y + (scaled_rect.height / 2) as i32,
                scale,
                self.text_color,
            );
        }
        
        // Draw arrows
        let arrow_size = (10.0 * scale) as u32;
        let arrow_padding = (5.0 * scale) as i32;
        
        // Left arrow
        let left_arrow_x = scaled_rect.x + arrow_padding;
        let left_arrow_y = scaled_rect.y + (scaled_rect.height / 2) as i32 - (arrow_size / 2) as i32;
        
        let left_arrow = &UI_CONFIG.widgets.selector.sprites.left_arrow;
        let left_src = Rect::from_array(*left_arrow);
        let left_dest = Rect::new(left_arrow_x, left_arrow_y, arrow_size, arrow_size);
        graphics.draw_texture("resources/sprites/gui.png", left_src.to_sdl(), left_dest.to_sdl());
        
        // Right arrow
        let right_arrow_x = scaled_rect.x + scaled_rect.width as i32 - arrow_size as i32 - arrow_padding;
        let right_arrow_y = scaled_rect.y + (scaled_rect.height / 2) as i32 - (arrow_size / 2) as i32;
        
        let right_arrow = &UI_CONFIG.widgets.selector.sprites.right_arrow;
        let right_src = Rect::from_array(*right_arrow);
        let right_dest = Rect::new(right_arrow_x, right_arrow_y, arrow_size, arrow_size);
        graphics.draw_texture("resources/sprites/gui.png", right_src.to_sdl(), right_dest.to_sdl());
    }
} 