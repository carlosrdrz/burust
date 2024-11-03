use crate::{graphics::Graphics, renderer::Renderer, ui::pane::{Pane, WidgetWrapper}, ui::{Drawable, Widget}};

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

    pub fn get_pane_widget(&mut self, pane_id: usize, widget_id: usize) -> &mut WidgetWrapper {
        return self.get_pane(pane_id).get_widget(widget_id)
    }

    pub fn get_pane_widget_as<T>(&mut self, pane_id: usize, widget_id: usize) -> &mut T
    where
        T: Widget + 'static,
    {
        return self.get_pane_widget(pane_id, widget_id).as_widget::<T>()
    }
}

impl Renderer for UIManager {
    fn render(&self, _layer: u16, graphics: &mut Graphics) {
        for pane in self.panes.iter() {
            pane.draw(graphics);
        }
    }
}
