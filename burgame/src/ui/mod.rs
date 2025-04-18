mod config;
mod layout;
mod renderer;

pub mod pane;
pub mod square;

use burengine::graphics::Graphics;
use as_any::AsAny;

pub trait Draw {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext);
}

pub struct DrawingContext {
    pub scale: f32,
    pub parent_x: i32,
    pub parent_y: i32,
}

impl Clone for DrawingContext {
    fn clone(&self) -> Self {
        Self {
            scale: self.scale,
            parent_x: self.parent_x,
            parent_y: self.parent_y,
        }
    }
}

pub trait Widget: Draw + AsAny {}

pub struct WidgetBox(pub Box<dyn Widget>);

impl WidgetBox {
    pub fn as_mut_widget<T>(&mut self) -> &mut T
    where
        T: Widget + 'static,
    {
        let widget = &mut *self.0;
        widget.as_any_mut().downcast_mut::<T>().unwrap()
    }
}

impl Draw for WidgetBox {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        self.0.draw(graphics, context);
    }
}
