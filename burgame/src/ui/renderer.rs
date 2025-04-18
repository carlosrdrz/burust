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

    fn scale_rect(&self, rect: Rect, scale: f32) -> Rect {
        Rect::new(
            (rect.x as f32 * scale) as i32,
            (rect.y as f32 * scale) as i32,
            (rect.width() as f32 * scale) as u32,
            (rect.height() as f32 * scale) as u32
        )
    }

    fn rect_from_array(&self, arr: [i32; 4]) -> Rect {
        Rect::new(arr[0], arr[1], arr[2] as u32, arr[3] as u32)
    }

    pub fn draw(&self, graphics: &mut Graphics, config: &PaneConfig, scale: f32) {
        // Draw background
        let bg_rect = self.scale_rect(self.layout.get_background_rect(config), scale);
        graphics.draw_rect(bg_rect, self.color);

        // Draw borders
        let (top_border, bottom_border) = self.layout.get_horizontal_border_rects(config);
        let (left_border, right_border) = self.layout.get_vertical_border_rects(config);
        
        let vertical_src = self.rect_from_array(config.sprites.vertical_border);
        let horizontal_src = self.rect_from_array(config.sprites.horizontal_border);
        
        graphics.draw_texture("resources/sprites/gui.png", vertical_src, self.scale_rect(top_border, scale));
        graphics.draw_texture("resources/sprites/gui.png", vertical_src, self.scale_rect(bottom_border, scale));
        graphics.draw_texture("resources/sprites/gui.png", horizontal_src, self.scale_rect(left_border, scale));
        graphics.draw_texture("resources/sprites/gui.png", horizontal_src, self.scale_rect(right_border, scale));

        // Draw corners
        let corner_sprites = [
            &config.sprites.top_left,
            &config.sprites.top_right,
            &config.sprites.bottom_left,
            &config.sprites.bottom_right,
        ];
        
        for i in 0..4 {
            let corner_rect = self.layout.get_corner_rect(i, config);
            let src = Rect::new(
                corner_sprites[i][0],
                corner_sprites[i][1],
                corner_sprites[i][2] as u32,
                corner_sprites[i][3] as u32
            );
            graphics.draw_texture("resources/sprites/gui.png", src, self.scale_rect(corner_rect, scale));
        }
    }
} 