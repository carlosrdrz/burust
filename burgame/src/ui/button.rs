use burengine::{graphics::Graphics, types::{Color, Rect}};
use crate::ui::config::UI_CONFIG;

use super::{Draw, DrawingContext, Widget};

pub struct Button {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub text: String,
    background_color: Color,
    text_color: Color,
    is_pressed: bool,
}

impl Widget for Button {}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text: text.to_string(),
            background_color: Color::from_array(UI_CONFIG.widgets.button.defaults.background_color),
            text_color: Color::from_array(UI_CONFIG.widgets.button.defaults.text_color),
            is_pressed: false,
        }
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.is_pressed = pressed;
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

impl Draw for Button {
    fn draw(&self, graphics: &mut Graphics, context: &DrawingContext) {
        let scale = context.scale;
        let x = self.x + context.parent_x;
        let y = self.y + context.parent_y;
        
        // Create base rect and scale it
        let base_rect = Rect::new(x, y, self.width, self.height);
        let scaled_rect = base_rect.scale(scale);
        
        // Draw background
        let bg_color = if self.is_pressed {
            // Darken color when pressed
            let r = (self.background_color.r as f32 * 0.8) as u8;
            let g = (self.background_color.g as f32 * 0.8) as u8;
            let b = (self.background_color.b as f32 * 0.8) as u8;
            Color::RGBA(r, g, b, self.background_color.a)
        } else {
            self.background_color
        };
        
        // Draw button background with rounded corners
        let button_sprite = &UI_CONFIG.widgets.button.sprites;
        
        // Draw the button background
        let src = Rect::from_array(button_sprite.background);
        graphics.draw_texture("resources/sprites/gui.png", src.to_sdl(), scaled_rect.to_sdl());
        
        // Draw text centered on button
        let text_scale = scale * 1.0;
        graphics.draw_text(
            &self.text,
            scaled_rect.x + (scaled_rect.width / 2) as i32,
            scaled_rect.y + (scaled_rect.height / 2) as i32,
            text_scale,
            self.text_color,
        );
    }
} 