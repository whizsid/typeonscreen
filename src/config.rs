use serde::{Deserialize,Serialize};
use std::default::Default;
use confy::{load,ConfyError};

#[derive( Serialize, Deserialize, Debug)]
pub struct Config {
    pub font_family: String,
    pub background_color: String,
    pub font_size: String,
    pub color: String,
    pub clear_text: bool,
    pub font_weight: i32,
}

impl Default for Config {
    fn default()->Config {
        Config {
            font_family: String::from("monospace"),
            background_color: String::from("rgba(27, 42, 61, 0.85)"),
            font_size: String::from("48px"),
            color: String::from("#fff"),
            clear_text: true,
            font_weight: 800
        }
    }
}

impl Config {
    pub fn load() -> Result<Config, ConfyError> {
        load("typeonscreen")
    }
}
