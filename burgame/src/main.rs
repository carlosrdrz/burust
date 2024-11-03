mod example_game;

use std::time::Duration;

use burengine::{self, types::{Dimensions, Rect}, ui::pane::Pane, ui::square::SquareWidget, Color};

pub fn main() {
    let game = Box::new(example_game::ExampleGame::new());
    let mut engine = burengine::engine::Engine::new(game, Dimensions::new(120, 120));

    {
        let color = Color::RGB(23, 23, 23);
        let position = Rect::new(10, 10, 100, 100);
        let widget = SquareWidget::new(color, position);

        let mut pane = Pane::new();
        pane.add_widget(Box::new(widget));

        let ui_manager = engine.get_ui_manager();
        ui_manager.add_pane(pane);
    }

    let mut i = 0;
    while !engine.is_done() {
        i = (i + 1) % 255;

        let ui_manager = engine.get_ui_manager();
        let square_widget: &mut SquareWidget = ui_manager.get_pane_widget_as(0, 0);
        square_widget.set_color(Color::RGB(i, 64, 255 - i));
        
        engine.run_loop();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
