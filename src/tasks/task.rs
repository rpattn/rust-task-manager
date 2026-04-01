use core::fmt;
use chrono::{DateTime, Utc};
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Priority {
    Low,
    Medium,
    High
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Status {
    Todo,
    Complete
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
        write!(f, "{}, {}, Priority: {:?}, Status: {:?}, Created at: {}",
            self.id, self.title, self.priority, self.done, self.created)
    }
}

impl Task {
    pub fn mark_complete(&mut self) {
        match self.done {
            Status::Todo => {self.done = Status::Complete},
            Status::Complete => {},
        }
    }
    #[allow(dead_code)]
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
    #[allow(dead_code)]
    pub fn get_title(&self) -> &str {
        &self.title
    }
    #[allow(dead_code)]
    pub fn get_created_at(&self) -> &DateTime<Utc> {
        &self.created
    }
}
