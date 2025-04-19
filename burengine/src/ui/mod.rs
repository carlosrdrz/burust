mod config;
mod layout;
mod renderer;

pub mod pane;
pub mod square;
pub mod button;
pub mod image;
pub mod input_box;
pub mod label;
pub mod selector;
pub mod manager;

use crate::graphics::Graphics;
use config::UIConfig;
use as_any::AsAny;

pub trait Draw {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext);
}

#[derive(Clone)]
pub struct DrawingContext<'a> {
    pub scale: f32,
    pub parent_x: i32,
    pub parent_y: i32,
    pub config: &'a UIConfig,
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
