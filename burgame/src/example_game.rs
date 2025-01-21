use burengine::game::Game;

use UIManager;

pub struct ExampleGame {
    ui_manager: UIManager,
    finished: bool
}

impl ExampleGame {
    pub fn new() -> ExampleGame {
        Self {
            ui_manager: UIManager::new(Vec::new()),
            finished: false, 
        }
    }

    pub fn get_ui_manager(&mut self) -> &mut UIManager {
        &mut self.ui_manager
    }
}

impl Game for ExampleGame {
    fn init(&mut self) {

    }

    fn main_loop(&mut self) {

    }

    fn end(&mut self) {
        self.finished = true;
    }

    fn is_done(&self) -> bool {
        self.finished
    }
}