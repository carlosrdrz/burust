use log::error;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::types::{Dimensions, Rect, Color, SdlColor};
use crate::texture_manager::TextureManager;
use crate::text::TextRenderer;

pub struct Graphics {
    canvas: Box<Canvas<Window>>,
    texture_manager: TextureManager,
    text_renderer: TextRenderer,
}

impl Graphics {
    pub fn new(dimensions: Dimensions, video_subsystem: VideoSubsystem) -> Self {
        let window = video_subsystem.window("rust-sdl2 demo", dimensions.width, dimensions.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let text_renderer = TextRenderer::new("resources/pixel_font.ttf", 10)
            .expect("Failed to initialize text renderer");
        
        Self { 
            canvas: Box::new(canvas), 
            texture_manager: TextureManager::new(texture_creator),
            text_renderer,
        }
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.canvas.set_draw_color(color.to_sdl());
        self.canvas.fill_rect(rect.to_sdl()).unwrap();
    }

    pub fn draw_texture(&mut self, path: &str, src: Rect, dst: Rect) {
        let texture = self.texture_manager.get(path);
        match texture {
            Ok(texture) => {
                let _ = self.canvas.copy(&texture, src.to_sdl(), dst.to_sdl());
            }
            Err(err) => {
                error!("Couldn't load texture {path}: {err}");
            }
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(SdlColor::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, _scale: f32, color: Color) {
        if let Err(e) = self.text_renderer.render_text(&mut self.canvas, text, x, y, color.to_sdl()) {
            error!("Failed to render text: {}", e);
        }
    }

    pub fn draw_text_centered(&mut self, text: &str, x: i32, y: i32, _scale: f32, color: Color) {
        if let Err(e) = self.text_renderer.render_text_centered(&mut self.canvas, text, x, y, color.to_sdl()) {
            error!("Failed to render text: {}", e);
        }
    }

    pub fn draw_rect_outline(&mut self, rect: Rect, color: Color, thickness: i32) {
        self.draw_rect(Rect::new(rect.x, rect.y, rect.width, thickness as u32), color);
        self.draw_rect(Rect::new(rect.x, rect.y + rect.height as i32 - thickness, rect.width, thickness as u32), color);
        self.draw_rect(Rect::new(rect.x, rect.y, thickness as u32, rect.height), color);
        self.draw_rect(Rect::new(rect.x + rect.width as i32 - thickness, rect.y, thickness as u32, rect.height), color);
    }
}
