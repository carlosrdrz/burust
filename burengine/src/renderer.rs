use crate::graphics::Graphics;

pub trait Renderer {
    fn render(&self, layer: u16, graphics: &mut Graphics);
}
