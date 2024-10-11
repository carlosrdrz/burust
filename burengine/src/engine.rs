use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::game::Game;
use crate::graphics::Graphics;
use crate::renderer::Renderer;
use crate::types::Dimensions;
use crate::ui_manager::UIManager;

pub struct Engine {
    game: Box<dyn Game>,
    graphics: Graphics,
    ui_manager: UIManager,
    event_pump: EventPump,
}

impl Engine {
    pub fn new(game: Box<dyn Game>, window_dimensions: Dimensions) -> Engine {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let graphics = Graphics::new(window_dimensions, video_subsystem);
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            game: game,
            graphics: graphics,
            ui_manager: UIManager::new(Vec::new()),
            event_pump: event_pump,
        }
    }

    pub fn get_ui_manager(&mut self) -> &mut UIManager {
        &mut self.ui_manager
    }

    pub fn run_loop(&mut self) {
        self.graphics.clear();
        self.ui_manager.render(0, &mut self.graphics);
        self.graphics.present();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.game.end();
                },
                _ => {}
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.game.is_done()
    }
}