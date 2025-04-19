use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct UIConfig {
    pub widgets: WidgetConfigs,
}

#[derive(Debug, Deserialize)]
pub struct WidgetConfigs {
    pub pane: PaneConfig,
    pub button: ButtonConfig,
    pub label: LabelConfig,
    pub input_box: InputBoxConfig,
    pub selector: SelectorConfig,
}

#[derive(Debug, Deserialize)]
pub struct PaneConfig {
    pub dimensions: PaneDimensions,
    pub defaults: PaneDefaults,
    pub sprites: PaneSprites,
}

#[derive(Debug, Deserialize)]
pub struct PaneDefaults {
    pub background_color: [u8; 4],
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

#[derive(Debug, Deserialize)]
pub struct ButtonConfig {
    pub defaults: ButtonDefaults,
}

#[derive(Debug, Deserialize)]
pub struct ButtonDefaults {
    pub background_color: [u8; 4],
    pub text_color: [u8; 4],
}

#[derive(Debug, Deserialize)]
pub struct LabelConfig {
    pub defaults: LabelDefaults,
}

#[derive(Debug, Deserialize)]
pub struct LabelDefaults {
    pub text_color: [u8; 4],
}

#[derive(Debug, Deserialize)]
pub struct InputBoxConfig {
    pub defaults: InputBoxDefaults,
}

#[derive(Debug, Deserialize)]
pub struct InputBoxDefaults {
    pub background_color: [u8; 4],
    pub text_color: [u8; 4],
    pub placeholder_color: [u8; 4],
    pub border_color: [u8; 4],
}

#[derive(Debug, Deserialize)]
pub struct SelectorConfig {
    pub defaults: SelectorDefaults,
    pub sprites: SelectorSprites,
}

#[derive(Debug, Deserialize)]
pub struct SelectorDefaults {
    pub text_color: [u8; 4],
}

#[derive(Debug, Deserialize)]
pub struct SelectorSprites {
    pub background: [i32; 4],
    pub left_arrow: [i32; 4],
    pub right_arrow: [i32; 4],
}

lazy_static::lazy_static! {
    pub static ref UI_CONFIG: UIConfig = {
        let config_str = fs::read_to_string("resources/ui_config.json")
            .expect("Failed to read UI config file");
        serde_json::from_str(&config_str)
            .expect("Failed to parse UI config file")
    };
} 