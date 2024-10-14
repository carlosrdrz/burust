use std::any::Any;

use sdl2::pixels::Color;

use crate::{graphics::Graphics, types::Rect};

pub trait Drawable {
    fn draw(&self, graphics: &mut Graphics);
}
pub trait Widget : Drawable {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub struct WidgetWrapper {
    widget: Box<dyn Widget>,
}

impl WidgetWrapper {
    pub fn as_widget<T>(&mut self) -> &mut T
    where
        T: Widget + 'static,
    {
        return self.widget.as_mut_any().downcast_mut::<T>().unwrap()
    }
}

impl Drawable for WidgetWrapper {
    fn draw(&self, graphics: &mut Graphics) {
        self.widget.draw(graphics);
    }
}

pub struct Pane {
    widgets: Vec<WidgetWrapper>,
}

impl Pane {
    pub fn new() -> Pane {
        Self { widgets: Vec::new() }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        let wrapper = WidgetWrapper { widget };
        self.widgets.push(wrapper)
    }

    pub fn get_widget(&mut self, id: usize) -> &mut WidgetWrapper {
        return self.widgets.get_mut(id).unwrap()
    }
}

impl Widget for Pane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Drawable for Pane {
    fn draw(&self, graphics: &mut Graphics) {
        for widget in self.widgets.iter() {
            widget.draw(graphics);
        }
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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Drawable for SquareWidget {
    fn draw(&self, graphics: &mut Graphics) {
        graphics.draw_rect(self.position, self.color);
    }
}
