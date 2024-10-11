use crate::{graphics::Graphics, renderer::Renderer, ui::pane::{Pane, Widget}};

pub struct UIManager {
    panes: Vec<Pane>,
}

impl UIManager {
    pub fn new(panes: Vec<Pane>) -> UIManager {
        Self { panes }
    }

    pub fn add_pane(&mut self, pane: Pane) {
        self.panes.push(pane)
    }

    pub fn get_pane(&mut self, id: usize) -> &mut Pane {
        let val = self.panes.get_mut(id).unwrap();
        &mut *val
    }
}

impl Renderer for UIManager {
    fn render(&self, layer: u16, graphics: &mut Graphics) {
        for pane in self.panes.iter() {
            pane.draw(graphics);
        }
    }
}