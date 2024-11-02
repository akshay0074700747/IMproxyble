use serde::Deserialize;
use std::collections::HashMap;
use std::error;

#[derive(Debug,Deserialize)]
pub struct Config {
    title: String,
    version: u32,
    settings: HashMap<String,serde_yaml::Value>
}

impl Config {
    pub fn new(contents: &str) -> Result<Config, Box<dyn error::Error>> {

        let config: Config = serde_yaml::from_str(contents)?;
        Ok(config)
    }
}