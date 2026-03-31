use core::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: i32,
    title: String,
    priority: Priority,
    done: Status,
    #[serde(with = "chrono::serde::ts_seconds")]
    created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Priority {
    Low,
    Medium,
    High
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Status {
    Todo,
    Complete
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: 0,
            title: String::from("New Task"),
            priority: Priority::Low,
            done: Status::Todo,
            created: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, Priority: {:?}, Status: {:?}, Created at: {}",
            self.id, self.title, self.priority, self.done, self.created)
    }
}
