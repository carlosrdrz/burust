extern crate sdl2;

pub mod utils;
pub mod engine;
pub mod game;
pub mod ui;
pub mod types;

mod ui_manager;
mod graphics;
mod renderer;
mod texture_manager;

pub use sdl2::pixels::Color;