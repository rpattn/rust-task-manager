use std::path::PathBuf;

use dirs::{config_local_dir, data_local_dir};
use serde::{Deserialize, Serialize};
use toml::ser::Error;

use crate::{
    store::{load, save},
    tasks::taskstore::QueryOptions,
};

pub struct Config {
    pub tasks_filename: String,
    config_dir: PathBuf,
    tasks_dir: PathBuf,
    pub query_options: QueryOptions,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    tasks_filename: String,
    query_options: QueryOptions,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("parse error: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("deserialise error: {0}")]
    SerialiseError(#[from] Error),
}

impl Default for Config {
    fn default() -> Self {
        let mut config_dir = config_local_dir().unwrap();
        config_dir.push(env!("CARGO_PKG_NAME"));

        let mut tasks_dir = data_local_dir().unwrap();
        tasks_dir.push(env!("CARGO_PKG_NAME"));

        Config {
            tasks_filename: String::from("tasks.json"),
            config_dir,
            tasks_dir,
            query_options: QueryOptions::default(),
        }
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            tasks_filename: String::from("tasks.json"),
            query_options: QueryOptions::default(),
        }
    }
}

impl Config {
    pub fn load_config(&mut self) -> Result<(), ConfigError> {
        match load(&self.get_config_filepath()) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => self.save()?,
            Err(e) => return Err(e.into()),
            Ok(s) if s.is_empty() => return Ok(()),
            Ok(s) => {
                let config_from_file: ConfigFile = toml::from_str(&s)?;
                self.tasks_filename = config_from_file.tasks_filename;
                self.query_options = config_from_file.query_options;
            }
        };
        Ok(())
    }
    pub fn save(&self) -> Result<(), ConfigError> {
        let default_config_file = ConfigFile::default();
        let config_toml = toml::to_string_pretty(&default_config_file)?;
        save(&self.get_config_filepath(), &config_toml)?;
        Ok(())
    }
    pub fn get_tasks_filepath(&self) -> PathBuf {
        self.tasks_dir.join(&self.tasks_filename)
    }
    pub fn change_tasks_filename(&mut self, file_name: &str) {
        self.tasks_filename = file_name.to_string();
    }
    pub fn get_config_filepath(&self) -> PathBuf {
        self.config_dir.join(PathBuf::from("config.toml"))
    }
}
