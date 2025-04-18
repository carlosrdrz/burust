use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct UIConfig {
    pub widgets: WidgetConfigs,
}

#[derive(Debug, Deserialize)]
pub struct WidgetConfigs {
    pub pane: PaneConfig,
}

#[derive(Debug, Deserialize)]
pub struct PaneConfig {
    pub dimensions: PaneDimensions,
    pub default_color: [u8; 4],
    pub sprites: PaneSprites,
}

#[derive(Debug, Deserialize)]
pub struct PaneDimensions {
    pub corner_size: u32,
    pub border_offset: i32,
    pub bg_padding: i32,
}

#[derive(Debug, Deserialize)]
pub struct PaneSprites {
    pub vertical_border: [i32; 4],
    pub horizontal_border: [i32; 4],
    pub top_left: [i32; 4],
    pub top_right: [i32; 4],
    pub bottom_left: [i32; 4],
    pub bottom_right: [i32; 4],
}

lazy_static::lazy_static! {
    pub static ref UI_CONFIG: UIConfig = {
        let config_str = fs::read_to_string("resources/ui_config.json")
            .expect("Failed to read UI config file");
        serde_json::from_str(&config_str)
            .expect("Failed to parse UI config file")
    };
} 