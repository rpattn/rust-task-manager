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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Status {
    Todo,
    Complete,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "todo"),
            Status::Complete => write!(f, "done")
        }
    }
}

#[derive(Debug)]
pub struct TaskEdit {
    pub title: Option<String>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
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
    // Can just use edit, but I like this fn
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
        if let Some(status) = fields.status {
            self.done = status;
        }
    }
    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created
    }
}
