use std::any::Any;

use sdl2::pixels::Color;

use crate::{graphics::Graphics, types::Rect};

pub trait Widget {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn draw(&self, graphics: &mut Graphics);
}

pub struct Pane {
  widgets: Vec<Box<dyn Widget>>,
}

impl Pane {
    pub fn new() -> Pane {
        Self { widgets: Vec::new() }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget)
    }

    pub fn get_widget(&mut self, id: usize) -> &mut dyn Widget {
        let val = self.widgets.get_mut(id).unwrap();
        &mut **val
    }
}

impl Widget for Pane {
    fn draw(&self, graphics: &mut Graphics) {
        for widget in self.widgets.iter() {
            widget.draw(graphics);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

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
    fn draw(&self, graphics: &mut Graphics) {
        graphics.draw_rect(self.position, self.color);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}