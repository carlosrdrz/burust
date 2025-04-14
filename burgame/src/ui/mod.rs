pub mod pane;
pub mod square;

use std::any::Any;
use burengine::graphics::Graphics;

pub trait Drawable {
    fn draw(&self, graphics: &mut Graphics);
}
pub trait Widget : Drawable {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}
