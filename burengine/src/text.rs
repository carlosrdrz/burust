use lazy_static::lazy_static;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

lazy_static! {
    static ref TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

pub struct TextRenderer {
    font: Font<'static, 'static>,
}

impl TextRenderer {
    pub fn new(font_path: &str, font_size: u16) -> Result<Self, String> {
        let font = TTF_CONTEXT.load_font(font_path, font_size).map_err(|e| e.to_string())?;
        
        Ok(TextRenderer { font })
    }

    pub fn render_text(&self, canvas: &mut Canvas<Window>, text: &str, x: i32, y: i32, color: Color) -> Result<(), String> {
        let surface = self.font.render(text)
            .blended(color)
            .map_err(|e| e.to_string())?;
        
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        
        let query = texture.query();
        let target = Rect::new(x, y, query.width, query.height);
        
        canvas.copy(&texture, None, target)?;
        Ok(())
    }
} 