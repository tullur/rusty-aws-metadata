use core::fmt;

use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Server {
    pub host: String,
    pub port: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub server: Server
}

impl Settings {
    pub fn new(name: &str) -> Result<Self, ConfigError> {
        let file: String = format!("./config/{}", name);
        let settings: Config = Config::builder()
            .add_source(File::with_name(&file))
            .build()?;

        settings.try_deserialize()
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", &self.host, &self.port)
    }
}