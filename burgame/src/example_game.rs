use burengine::game::Game;

use burengine::{self, types::Rect, Color};
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

    pub fn get_ui_manager(&mut self) -> &mut UIManager {
        &mut self.ui_manager
    }
}

impl Game for ExampleGame {
    fn init(&mut self) {
        let color = Color::RGB(23, 23, 23);
        let position = Rect::new(10, 10, 100, 100);
        let widget = SquareWidget::new(color, position);

        let mut pane = Pane::new();
        pane.add_widget(Box::new(widget));
        self.ui_manager.add_pane(pane);
    }

    fn main_loop(&mut self) {
        self.i = (self.i + 1) % 255;
        let square_widget: &mut SquareWidget = self.ui_manager.get_pane_widget_as_mut(0, 0);
        square_widget.set_color(Color::RGB(self.i, 64, 255 - self.i));
    }

    fn end(&mut self) {
        self.finished = true;
    }

    fn is_done(&self) -> bool {
        self.finished
    }
}
