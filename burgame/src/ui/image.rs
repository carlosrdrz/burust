use burengine::{graphics::Graphics, types::Rect};

use super::{Draw, DrawingContext, Widget};

pub struct Image {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    texture_path: String,
    src_rect: Option<Rect>,
}

impl Widget for Image {}

impl Image {
    pub fn new(x: i32, y: i32, width: u32, height: u32, texture_path: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            texture_path: texture_path.to_string(),
            src_rect: None,
        }
    }
    
    #[allow(dead_code)]
    pub fn set_texture(&mut self, texture_path: &str) {
        self.texture_path = texture_path.to_string();
    }
    
    #[allow(dead_code)]
    pub fn set_src_rect(&mut self, src_rect: Option<Rect>) {
        self.src_rect = src_rect;
    }
}

impl Draw for Image {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let scale = context.scale;
        let x = self.x + context.parent_x;
        let y = self.y + context.parent_y;
        
        // Create base rect and scale it
        let base_rect = Rect::new(x, y, self.width, self.height);
        let dest_rect = base_rect.scale(scale);
        
        // Use the provided source rect or use the entire texture
        let src_rect = match self.src_rect {
            Some(rect) => rect,
            None => Rect::new(0, 0, self.width, self.height),
        };
        
        // Draw the image
        graphics.draw_texture(&self.texture_path, src_rect, dest_rect);
    }
} 