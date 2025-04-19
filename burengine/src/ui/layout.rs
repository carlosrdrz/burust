use crate::types::Rect;
use super::config::PaneConfig;

pub struct PaneLayout {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl PaneLayout {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    pub fn get_background_rect(&self, config: &PaneConfig) -> Rect {
        let padding = config.dimensions.bg_padding;
        Rect::new(
            self.x + padding,
            self.y + padding,
            self.width - (padding as u32 * 2),
            self.height - (padding as u32 * 2)
        )
    }

    pub fn get_horizontal_border_rects(&self, config: &PaneConfig) -> (Rect, Rect) {
        let border_offset = config.dimensions.border_offset;
        let corner_size = config.dimensions.corner_size;
        
        let x = self.x + border_offset;
        let top_y = self.y - border_offset;
        let bottom_y = self.y + self.height as i32 - border_offset;
        let size = self.width - corner_size;
        let height = corner_size;

        (
            Rect::new(x, top_y, size, height),
            Rect::new(x, bottom_y, size, height)
        )
    }

    pub fn get_vertical_border_rects(&self, config: &PaneConfig) -> (Rect, Rect) {
        let border_offset = config.dimensions.border_offset;
        let corner_size = config.dimensions.corner_size;
        
        let y = self.y + border_offset;
        let left_x = self.x - border_offset;
        let right_x = self.x + self.width as i32 - border_offset;
        let size = self.height - corner_size;
        let width = corner_size;

        (
            Rect::new(left_x, y, width, size),
            Rect::new(right_x, y, width, size)
        )
    }

    pub fn get_corner_rect(&self, corner_index: usize, config: &PaneConfig) -> Rect {
        let corner_size = config.dimensions.corner_size;
        let border_offset = config.dimensions.border_offset;
        
        let (x, y) = match corner_index {
            0 => (self.x, self.y),  // Top-left
            1 => (self.x + self.width as i32 - corner_size as i32, self.y),  // Top-right
            2 => (self.x, self.y + self.height as i32 - corner_size as i32),  // Bottom-left
            3 => (self.x + self.width as i32 - corner_size as i32,
                  self.y + self.height as i32 - corner_size as i32),  // Bottom-right
            _ => panic!("Invalid corner index"),
        };
        
        let offsets = match corner_index {
            0 => [-border_offset, -border_offset],
            1 => [border_offset, -border_offset],
            2 => [-border_offset, border_offset],
            3 => [border_offset, border_offset],
            _ => panic!("Invalid corner index"),
        };
        
        Rect::new(
            x + offsets[0],
            y + offsets[1],
            corner_size,
            corner_size
        )
    }
} 