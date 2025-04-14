use std::collections::HashMap;

use sdl2::{image::LoadTexture, render::{Texture, TextureCreator}, video::WindowContext};

pub struct TextureManager<'a> {
    texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<String, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self { texture_creator, cache: HashMap::new() }
    }

    pub fn get(&mut self, path: &str) -> Result<&Texture<'a>, String> {
        if !self.cache.contains_key(path) {
            let texture = self.texture_creator.load_texture(path)?;
            self.cache.insert(path.to_string(), texture);
        }
        Ok(self.cache.get(path).unwrap())
    }
}
