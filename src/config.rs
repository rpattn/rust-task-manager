use std::{fmt, path::PathBuf};

use dirs::{config_local_dir, data_local_dir};
use serde::{Deserialize, Serialize};
use toml::ser::Error;

use crate::{
    store::{load, save},
    tasks::{taskstore::QueryOptions},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub store_type: StoreType,
    pub tasks_filename: String,
    #[serde(skip)]
    pub config_filename: String,
    #[serde(skip)]
    config_dir: PathBuf,
    #[serde(skip)]
    tasks_dir: PathBuf,
    pub query_options: QueryOptions,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum StoreType {
    BasicStore,
    JsonStore,
}

pub struct ConfigFields {
    pub tasks_filename: Option<String>,
    pub config_filename: Option<String>,
    pub query_options: QueryOptions,
}

impl Default for Config {
    fn default() -> Self {
        let mut config_dir = config_local_dir().unwrap();
        config_dir.push(env!("CARGO_PKG_NAME"));

        let mut tasks_dir = data_local_dir().unwrap();
        tasks_dir.push(env!("CARGO_PKG_NAME"));

        Config {
            store_type: StoreType::BasicStore,
            tasks_filename: String::from("tasks.json"),
            config_filename: String::from("config.toml"),
            config_dir,
            tasks_dir,
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
                let config_from_file: Config = toml::from_str(&s)?;
                self.tasks_filename = config_from_file.tasks_filename;
                self.query_options = config_from_file.query_options;
            }
        };
        Ok(())
    }
    pub fn save(&self) -> Result<(), ConfigError> {
        let config_toml = toml::to_string_pretty(&self)?;
        save(&self.get_config_filepath(), &config_toml)?;
        Ok(())
    }
    pub fn get_tasks_filepath(&self) -> PathBuf {
        self.tasks_dir.join(&self.tasks_filename)
    }
    pub fn get_config_filepath(&self) -> PathBuf {
        self.config_dir.join(&self.config_filename)
    }
    pub fn update_config(&mut self, config_fields: ConfigFields) -> Result<(), ConfigError> {
        if let Some(filename) = config_fields.tasks_filename {
            self.tasks_filename = filename;
        }
        if let Some(filename) = config_fields.config_filename {
            self.config_filename = filename;
        }
        if let Some(option) = config_fields.query_options.page {
            self.query_options.page = Some(option);
        }
        if let Some(option) = config_fields.query_options.page_size {
            self.query_options.page_size = Some(option);
        }
        if let Some(option) = config_fields.query_options.sort_field {
            self.query_options.sort_field = Some(option);
        }
        if let Some(option) = config_fields.query_options.sort_order {
            self.query_options.sort_order = Some(option);
        }
        if let Some(option) = config_fields.query_options.filter {
            self.query_options.filter = Some(option);
        }
        if let Some(option) = config_fields.query_options.value {
            self.query_options.value = Some(option);
        }
        self.save()
    }
    pub fn clear_filter(&mut self) -> Result<(), ConfigError> {
        self.query_options.value = None;
        self.query_options.filter = None;
        self.save()
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config File: {}
Tasks File: {}
Query Options: \n{}",
            self.config_filename, self.tasks_filename, self.query_options
        )
    }
}
