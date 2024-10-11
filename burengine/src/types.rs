pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    pub fn new(width: u32, height: u32) -> Dimensions {
        Self { width, height }
    }
}

pub use sdl2::rect::Point;
pub use sdl2::rect::Rect;