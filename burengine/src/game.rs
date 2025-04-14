use crate::graphics::Graphics;

pub trait Game {
    fn init(&mut self);
    fn end(&mut self);
    fn render(&self, graphics: &mut Graphics);
    fn main_loop(&mut self);
    fn is_done(&self) -> bool;
}
