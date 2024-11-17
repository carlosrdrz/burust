use log::error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::types::Dimensions;
use crate::texture_manager::TextureManager;

pub struct Graphics {
    canvas: Box<Canvas<Window>>,
    texture_manager: TextureManager<'static>,
}

impl Graphics {
    pub fn new(dimensions: Dimensions, video_subsystem: VideoSubsystem) -> Self {
        let window = video_subsystem.window("rust-sdl2 demo", dimensions.width, dimensions.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self { canvas: Box::new(canvas), texture_manager: TextureManager::new(texture_creator) }
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn draw_texture(&'static mut self, path: &str, src: Rect, dst: Rect) {
        let texture = self.texture_manager.get(path);
        match texture {
            Ok(texture) => {
                let _ = self.canvas.copy(&texture, src, dst);
            }
            Err(err) => {
                error!("Couldn't load texture {path}: {err}");
            }
        } 
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}