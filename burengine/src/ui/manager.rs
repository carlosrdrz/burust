use crate::{graphics::Graphics, renderer::Renderer};
use crate::{ui::pane::Pane, ui::{Widget, Draw, WidgetBox, DrawingContext}};

use super::config::UIConfig;

pub struct Manager {
    panes: Vec<Pane>,
    widgets: Vec<WidgetBox>,
    config: UIConfig,
}

impl Manager {
    pub fn new(config_path: &str) -> Self {
        Self { panes: Vec::new(), widgets: Vec::new(), config: UIConfig::load(config_path) }
    }

    pub fn add_pane(&mut self, pane: Pane) {
        self.panes.push(pane)
    }

    #[allow(dead_code)]
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        let wrapper = WidgetBox(widget);
        self.widgets.push(wrapper)
    }

    pub fn get_pane_mut(&mut self, id: usize) -> &mut Pane {
        let val = self.panes.get_mut(id).unwrap();
        &mut *val
    }

    pub fn get_pane_widget_mut(&mut self, pane_id: usize, widget_id: usize) -> &mut WidgetBox {
        return self.get_pane_mut(pane_id).get_widget_mut(widget_id)
    }

    pub fn get_pane_widget_as_mut<T>(&mut self, pane_id: usize, widget_id: usize) -> &mut T
    where
        T: Widget + 'static,
    {
        return self.get_pane_widget_mut(pane_id, widget_id).as_mut_widget::<T>()
    }
}

impl Renderer for Manager {
    fn render(&self, _layer: u16, graphics: &mut Graphics) {
        let context = DrawingContext { parent_x: 0, parent_y: 0, scale: 1.0, config: &self.config };
        
        for pane in self.panes.iter() {
            pane.draw(graphics, &context);
        }

        for widget in self.widgets.iter() {
            widget.draw(graphics, &context);
        }
    }
}
