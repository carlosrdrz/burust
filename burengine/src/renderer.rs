use crate::graphics::Graphics;

pub trait Renderer {
    fn render<'a>(&self, layer: u16, graphics: &mut Graphics<'a>);
}
