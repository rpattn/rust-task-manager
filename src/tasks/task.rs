use chrono::{DateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Task {
    id: Uuid,
    pub title: String,
    pub priority: Priority,
    pub done: Status,
    #[serde(with = "chrono::serde::ts_seconds")]
    created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, clap::ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Todo,
    Complete,
}

#[derive(Debug)]
pub struct TaskEdit {
    pub title: Option<String>,
    pub priority: Option<Priority>,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: Uuid::new_v4(),
            title: String::from("New Task"),
            priority: Priority::Low,
            done: Status::Todo,
            created: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, Priority: {:?}, Status: {:?}, Created at: {}",
            self.id, self.title, self.priority, self.done, self.created
        )
    }
}

impl Task {
    pub fn mark_complete(&mut self) {
        self.done = Status::Complete;
    }
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
    pub fn edit(&mut self, fields: TaskEdit) {
        if let Some(title) = fields.title {
            self.title = title;
        }
        if let Some(priority) = fields.priority {
            self.priority = priority;
        }
    }
}
