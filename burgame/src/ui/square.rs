use std::any::Any;

use burengine::{graphics::Graphics, types::Rect, Color};

use super::{Drawable, Widget};

pub struct SquareWidget {
    color: Color,
    position: Rect,
}

impl SquareWidget {
    pub fn new(color: Color, position: Rect) -> SquareWidget {
        Self { color, position }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Widget for SquareWidget {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Drawable for SquareWidget {
    fn draw<'a>(&self, graphics: &mut Graphics<'a>) {
        graphics.draw_rect(self.position, self.color);
    }
}
