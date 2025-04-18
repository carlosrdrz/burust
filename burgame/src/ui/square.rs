use burengine::{graphics::Graphics, types::{Color, Rect}};

use super::{Draw, DrawingContext, Widget};

pub struct SquareWidget {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    color: Color,
}

impl SquareWidget {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> SquareWidget {
        Self { x, y, width, height, color }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Widget for SquareWidget {}
impl Draw for SquareWidget {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let x = self.x + context.parent_x;
        let y = self.y + context.parent_y;
        
        let rect = Rect::new(x, y, self.width, self.height);
        graphics.draw_rect(rect, self.color);
    }
}
