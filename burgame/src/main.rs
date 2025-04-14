mod example_game;
mod ui;
mod ui_manager;

use std::time::Duration;
use burengine::types::Dimensions;

static SCREEN_WIDTH: u32 = 1280;
static SCREEN_HEIGHT: u32 = 720;

pub fn main() {
    let game = Box::new(example_game::ExampleGame::new());
    let mut engine = burengine::engine::Engine::new(game, Dimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT));

    while !engine.is_done() {
        engine.run_loop();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
