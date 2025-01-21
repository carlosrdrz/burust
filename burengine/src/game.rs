pub trait Game {
    fn init(&mut self);
    fn end(&mut self);
    fn main_loop(&mut self);
    fn is_done(&self) -> bool;
}