use burengine::game::Game;

pub struct ExampleGame {
    finished: bool
}

impl ExampleGame {
    pub fn new() -> ExampleGame {
        Self { finished: false }
    }
}

impl Game for ExampleGame {
    fn init(&mut self) {

    }

    fn end(&mut self) {
        self.finished = true;
    }

    fn is_done(&self) -> bool {
        self.finished
    }
}