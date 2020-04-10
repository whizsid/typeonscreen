use serde::{Deserialize,Serialize};
use std::default::Default;
use confy::{load,ConfyError};

#[derive( Serialize, Deserialize, Debug)]
pub struct Config {
    pub font_family: String,
    pub background_color: String,
    pub font_size: String,
    pub color: String,
    pub clear_text: bool
}

impl Default for Config {
    fn default()->Config {
        Config {
            font_family: String::from("monospace"),
            background_color: String::from("rgba(0, 250, 250, 0.7)"),
            font_size: String::from("40px"),
            color: String::from("#ff0000"),
            clear_text: true
        }
    }
}

impl Config {
    pub fn load() -> Result<Config, ConfyError> {
        load("typeonscreen")
    }
}
