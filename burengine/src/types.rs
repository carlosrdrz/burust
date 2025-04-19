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
pub use sdl2::pixels::Color as SdlColor;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    pub fn scale(&self, scale: f32) -> Self {
        Self {
            x: (self.x as f32 * scale) as i32,
            y: (self.y as f32 * scale) as i32,
            width: (self.width as f32 * scale) as u32,
            height: (self.height as f32 * scale) as u32,
        }
    }

    pub fn from_array(arr: [i32; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            width: arr[2] as u32,
            height: arr[3] as u32,
        }
    }

    // Convert to SDL2 Rect
    pub fn to_sdl(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x, self.y, self.width, self.height)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn from_array(rgba: [u8; 4]) -> Self {
        Self {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        }
    }
    
    // Convert to SDL2 Color
    pub fn to_sdl(self) -> SdlColor {
        SdlColor::RGBA(self.r, self.g, self.b, self.a)
    }
}