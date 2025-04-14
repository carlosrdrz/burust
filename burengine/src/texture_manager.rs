use std::collections::HashMap;

use sdl2::{image::LoadTexture, render::{Texture, TextureCreator}, video::WindowContext};

pub struct TextureManager {
    texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self { texture_creator, cache: HashMap::new() }
    }

    pub fn get(&mut self, path: &str) -> Result<&Texture, String> {
        if !self.cache.contains_key(path) {
            let texture = self.texture_creator.load_texture(path)?;
            self.cache.insert(path.to_string(), texture);
        }
        Ok(self.cache.get(path).unwrap())
    }
}
