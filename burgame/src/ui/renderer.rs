use burengine::graphics::Graphics;
use burengine::types::{Rect, Color};
use super::config::PaneConfig;
use super::layout::PaneLayout;

pub struct PaneRenderer {
    layout: PaneLayout,
    color: Color,
}

impl PaneRenderer {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Self {
        Self {
            layout: PaneLayout::new(x, y, width, height),
            color,
        }
    }

    pub fn draw(&self, graphics: &mut Graphics, config: &PaneConfig, scale: f32) {
        // Draw background
        let bg_rect = self.layout.get_background_rect(config).scale(scale);
        graphics.draw_rect(bg_rect.to_sdl(), self.color);

        // Draw borders
        let (top_border, bottom_border) = self.layout.get_horizontal_border_rects(config);
        let (left_border, right_border) = self.layout.get_vertical_border_rects(config);
        
        let vertical_src = Rect::from_array(config.sprites.vertical_border);
        let horizontal_src = Rect::from_array(config.sprites.horizontal_border);
        
        graphics.draw_texture("resources/sprites/gui.png", vertical_src.to_sdl(), top_border.scale(scale).to_sdl());
        graphics.draw_texture("resources/sprites/gui.png", vertical_src.to_sdl(), bottom_border.scale(scale).to_sdl());
        graphics.draw_texture("resources/sprites/gui.png", horizontal_src.to_sdl(), left_border.scale(scale).to_sdl());
        graphics.draw_texture("resources/sprites/gui.png", horizontal_src.to_sdl(), right_border.scale(scale).to_sdl());

        // Draw corners
        let corner_sprites = [
            &config.sprites.top_left,
            &config.sprites.top_right,
            &config.sprites.bottom_left,
            &config.sprites.bottom_right,
        ];
        
        for i in 0..4 {
            let corner_rect = self.layout.get_corner_rect(i, config);
            let src = Rect::from_array(*corner_sprites[i]);
            graphics.draw_texture("resources/sprites/gui.png", src.to_sdl(), corner_rect.scale(scale).to_sdl());
        }
    }
} 