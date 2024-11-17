use std::{collections::HashMap, rc::Rc};

use sdl2::{image::LoadTexture, render::{Texture, TextureCreator}, video::WindowContext};

pub struct TextureManager<'a> {
    texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<String, Rc<Texture<'a>>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self { texture_creator, cache: HashMap::new() }
    }

    pub fn get(&'a mut self, path: &str) -> Result<Rc<Texture>, String> {
        self.cache.get(path).cloned().map_or_else(|| {
            let resource = Rc::new(self.texture_creator.load_texture(path)?);
            self.cache.insert(path.to_string(), Rc::clone(&resource));
            Ok(resource)
        }, Ok)
    }
}