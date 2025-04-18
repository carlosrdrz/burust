use burengine::game::Game;
use burengine::graphics::Graphics;
use burengine::renderer::Renderer;
use burengine::{self, types::Color};
use crate::{ui::pane::Pane, ui::square::SquareWidget};

use crate::ui_manager::UIManager;

pub struct ExampleGame {
    ui_manager: UIManager,
    finished: bool,
    i: u8,
}

impl ExampleGame {
    pub fn new() -> ExampleGame {
        Self {
            ui_manager: UIManager::new(Vec::new()),
            finished: false,
            i: 0,
        }
    }
}

impl Game for ExampleGame {
    fn init(&mut self) {
        let color = Color::RGB(23, 23, 23);
        let widget = SquareWidget::new(20, 20, 460, 460, color);

        let mut pane = Pane::new(10, 10, 500, 500);
        pane.add_widget(Box::new(widget));
        self.ui_manager.add_pane(pane);

        let color = Color::RGB(255, 255, 255);
        let widget = SquareWidget::new(530, 10, 200, 200, color);
        self.ui_manager.add_widget(Box::new(widget));
    }

    fn main_loop(&mut self) {
        self.i = (self.i + 1) % 255;
        let square_widget: &mut SquareWidget = self.ui_manager.get_pane_widget_as_mut(0, 0);
        square_widget.set_color(Color::RGB(self.i, 64, 255 - self.i));
    }

    fn render(&self, graphics: &mut Graphics) {
        self.ui_manager.render(0, graphics);
    }

    fn end(&mut self) {
        self.finished = true;
    }

    fn is_done(&self) -> bool {
        self.finished
    }
}
