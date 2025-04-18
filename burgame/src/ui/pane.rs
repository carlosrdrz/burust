use burengine::graphics::Graphics;
use burengine::types::Color;

use super::{Draw, Widget, WidgetBox, DrawingContext};
use super::config::UI_CONFIG;
use super::renderer::PaneRenderer;

pub struct Pane {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    color: Color,
    widgets: Vec<WidgetBox>,
}

impl Pane {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Pane {
        let bg_color = UI_CONFIG.widgets.pane.default_color;
        Self { 
            x, 
            y, 
            width, 
            height, 
            color: Color::RGBA(bg_color[0], bg_color[1], bg_color[2], bg_color[3]), 
            widgets: Vec::new() 
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        let wrapper = WidgetBox(widget);
        self.widgets.push(wrapper)
    }

    pub fn get_widget_mut(&mut self, id: usize) -> &mut WidgetBox {
        return self.widgets.get_mut(id).unwrap()
    }
}

impl Widget for Pane {}
impl Draw for Pane {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let scale = context.scale;
        
        // Create renderer and draw pane
        let renderer = PaneRenderer::new(self.x, self.y, self.width, self.height, self.color);
        renderer.draw(graphics, &UI_CONFIG.widgets.pane, scale);

        // Draw child widgets
        for widget in self.widgets.iter() {
            let mut pane_context = context.clone();
            pane_context.parent_x += self.x;
            pane_context.parent_y += self.y;
            widget.draw(graphics, &pane_context);
        }
    }
}
