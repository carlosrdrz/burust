use lazy_static::lazy_static;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;

lazy_static! {
    static ref TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

struct CachedText {
    texture: sdl2::render::Texture,
    width: u32,
    height: u32,
}

pub struct TextRenderer {
    font: Font<'static, 'static>,
    texture_cache: HashMap<String, CachedText>,
}

impl TextRenderer {
    pub fn new(font_path: &str, font_size: u16) -> Result<Self, String> {
        let font = TTF_CONTEXT.load_font(font_path, font_size).map_err(|e| e.to_string())?;
        
        Ok(TextRenderer { 
            font,
            texture_cache: HashMap::new(),
        })
    }

    pub fn render_text(&mut self, canvas: &mut Canvas<Window>, text: &str, x: i32, y: i32, color: Color) -> Result<(), String> {
        let cached_text = self.get_cached_text(canvas, text, color)?;
        let target = Rect::new(x, y, cached_text.width, cached_text.height);
        
        canvas.copy(&cached_text.texture, None, target)?;
        Ok(())
    }

    pub fn render_text_centered(&mut self, canvas: &mut Canvas<Window>, text: &str, x: i32, y: i32, color: Color) -> Result<(), String> {
        let cached_text = self.get_cached_text(canvas, text, color)?;
        let x_center = x - (cached_text.width as i32) / 2;
        let y_center = y - (cached_text.height as i32) / 2;
        let target = Rect::new(x_center, y_center, cached_text.width, cached_text.height);
        
        canvas.copy(&cached_text.texture, None, target)?;
        Ok(())
    }

    fn get_cached_text(&mut self, canvas: &mut Canvas<Window>, text: &str, color: Color) -> Result<&CachedText, String> {
        if !self.texture_cache.contains_key(text) {
            let surface = self.font.render(&text.to_uppercase())
                .blended(color)
                .map_err(|e| e.to_string())?;
            
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator.create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            
            let query = texture.query();
            let cached_text = CachedText {
                texture,
                width: query.width,
                height: query.height,
            };
            
            self.texture_cache.insert(text.to_string(), cached_text);
        }
        
        Ok(self.texture_cache.get(text).unwrap())
    }
} 