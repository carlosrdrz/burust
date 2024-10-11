//! This public module exposes a trait Config that is meant to be 
//! abstract so it can be switched between a file config or a constant
//! config used for tests. Unfortunately right now the actual implementation
//! is decided in this file instead of a user of the lib choosing how to load
//! the config.
//! 
//! TODO:
//! - Allow for users to choose how to read the config

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Box<dyn Config + Sync> = Box::new(ConstConfig::new(HashMap::from([
        ("fullscreen", "false"),
        ("scale", "1.0"),
    ])));
}

pub trait Config {
    fn get_bool(&self, key: &str) -> bool;
    fn get_float(&self, key: &str) -> f64;
}

pub struct ConstConfig {
    configs: HashMap<&'static str, &'static str>,
}

impl ConstConfig {
    pub fn new(c: HashMap<&'static str, &'static str>) -> ConstConfig {
        ConstConfig { configs: c }
    }
}

impl Config for ConstConfig {
    fn get_bool(&self, key: &str) -> bool {
        self.configs[key].parse::<bool>().unwrap()
    }

    fn get_float(&self, key: &str) -> f64 {
        self.configs[key].parse::<f64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bool() {
        let config = ConstConfig::new(HashMap::from([
            ("fullscreen", "false"),
            ("scale", "1.0"),
        ]));

        let result = config.get_bool("fullscreen");
        assert_eq!(result, false);
    }

    #[test]
    fn get_float() {
        let config = ConstConfig::new(HashMap::from([
            ("fullscreen", "false"),
            ("scale", "1.33"),
        ]));

        let result = config.get_float("scale");
        assert_eq!(result, 1.33);
    }
}
