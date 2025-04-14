use std::any::Any;

use burengine::graphics::Graphics;
use burengine::types::Rect;

use super::{Drawable, Widget};

pub struct WidgetWrapper {
    widget: Box<dyn Widget>,
}

impl WidgetWrapper {
    pub fn as_mut_widget<T>(&mut self) -> &mut T
    where
        T: Widget + 'static,
    {
        return self.widget.as_mut_any().downcast_mut::<T>().unwrap()
    }
}

impl Drawable for WidgetWrapper {
    fn draw<'a>(&self, graphics: &mut Graphics<'a>) {
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

    pub fn get_widget_mut(&mut self, id: usize) -> &mut WidgetWrapper {
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
    fn draw<'a>(&self, graphics: &mut Graphics<'a>) {
        graphics.draw_texture("test", Rect::new(0, 0, 10, 10), Rect::new(0, 0, 10, 10));

        for widget in self.widgets.iter() {
            widget.draw(graphics);
        }
    }
}
