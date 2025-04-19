use crate::graphics::Graphics;
use crate::types::{Rect, Color};

use super::DrawingContext;
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

    pub fn draw(&self, graphics: &mut Graphics, context: &DrawingContext, scale: f32) {
        // Draw background
        let pane_config = &context.config.widgets.pane;
        let bg_rect = self.layout.get_background_rect(pane_config).scale(scale);
        graphics.draw_rect(bg_rect, self.color);

        // Draw borders
        let (top_border, bottom_border) = self.layout.get_horizontal_border_rects(pane_config);
        let (left_border, right_border) = self.layout.get_vertical_border_rects(pane_config);
        
        let vertical_src = Rect::from_array(pane_config.sprites.vertical_border);
        let horizontal_src = Rect::from_array(pane_config.sprites.horizontal_border);
        
        graphics.draw_texture(&context.config.img_path, vertical_src, top_border.scale(scale));
        graphics.draw_texture(&context.config.img_path, vertical_src, bottom_border.scale(scale));
        graphics.draw_texture(&context.config.img_path, horizontal_src, left_border.scale(scale));
        graphics.draw_texture(&context.config.img_path, horizontal_src, right_border.scale(scale));

        // Draw corners
        let corner_sprites = [
            &pane_config.sprites.top_left,
            &pane_config.sprites.top_right,
            &pane_config.sprites.bottom_left,
            &pane_config.sprites.bottom_right,
        ];
        
        for i in 0..4 {
            let corner_rect = self.layout.get_corner_rect(i, pane_config);
            let src = Rect::from_array(*corner_sprites[i]);
            graphics.draw_texture(&context.config.img_path, src, corner_rect.scale(scale));
        }
    }
} 