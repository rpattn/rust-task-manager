use std::{path::PathBuf};

use dirs::{config_local_dir, data_local_dir};

use crate::tasks::taskstore::{QueryOptions, SortOrder, TaskField};

pub struct Config {
    pub tasks_filename: String,
    config_dir: PathBuf,
    tasks_dir: PathBuf,
    pub query_options: QueryOptions,
}

impl Default for Config {
    fn default() -> Self {
        let mut config_dir = config_local_dir().unwrap();
        config_dir.push(env!("CARGO_PKG_NAME"));

        let mut tasks_dir = data_local_dir().unwrap();
        tasks_dir.push(env!("CARGO_PKG_NAME"));

        Config {
            tasks_filename: String::from("tasks.json"),
            config_dir: config_dir,
            tasks_dir: tasks_dir,
            query_options: QueryOptions {
                page: 0usize,
                page_size: 5usize,
                sort_field: TaskField::Created,
                sort_order: SortOrder::Asc,
                filter: Some(TaskField::Status),
                value: Some(String::from("todo")),
            }
        }
    }
}

impl Config {
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
