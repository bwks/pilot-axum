use std::fs;

use anyhow::Result;

use serde::Deserialize;
use toml;

use crate::konst::CONFIG_FILENAME;

#[derive(Deserialize, Debug)]
pub struct PilotConfig {
    pub ipv4_listener: Ipv4Listener,
    pub logging: Logging,
    pub database: Database,
}

impl PilotConfig {
    pub fn load() -> Result<PilotConfig> {
        let filename = CONFIG_FILENAME;
        let contents = fs::read_to_string(filename)?;
        let pilot_config: PilotConfig = toml::from_str(&contents)?;

        Ok(pilot_config)
    }
}

#[derive(Deserialize, Debug)]
pub struct Ipv4Listener {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Logging {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name,
        )
    }
}

// pub fn get_configuration() -> Result<Settings, config::ConfigError> {
//     let settings = config::Config::builder()
//         .add_source(config::File::new(
//             "configuration.yaml",
//             config::FileFormat::Yaml,
//         ))
//         .build()?;

//     settings.try_deserialize::<Settings>()
// }
