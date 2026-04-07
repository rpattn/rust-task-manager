use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tasks::{Task, task::TaskEdit};

#[derive(Debug, thiserror::Error)]
pub enum TaskStoreError {
    #[error("task not found for id {id}")]
    TaskNotFound { id: GetBy },
    #[error("backend error: {0}")]
    BackendError(#[from] Box<dyn std::error::Error>),
}

pub trait TaskStore {
    fn get<B: IntoGetBy>(&self, by: B) -> Option<Task>;
    fn add(&mut self, task: Task);
    fn edit(&mut self, by: impl IntoGetBy, edit: TaskEdit) -> Result<(), TaskStoreError>;
    fn remove(&mut self, by: impl IntoGetBy) -> Result<(), TaskStoreError>;
    fn get_all(&self, page: Option<&QueryOptions>) -> Vec<Task>;
    fn clear_all_tasks(&mut self);
    fn open(&mut self) -> Result<(), TaskStoreError> {
        Ok(())
    }
    fn close(&mut self) -> Result<(), TaskStoreError> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum GetBy {
    ByIndex(usize),
    ByUuid(Uuid),
    Last,
}

impl Display for GetBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetBy::ByIndex(id) => write!(f, "{id}"),
            GetBy::ByUuid(id) => write!(f, "{id}"),
            GetBy::Last => write!(f, "last"),
        }
    }
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize, clap::ValueEnum)]
pub enum TaskField {
    Title,
    Priority,
    Created,
    Status,
}

impl fmt::Display for TaskField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize, clap::ValueEnum)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub sort_field: Option<TaskField>,
    pub sort_order: Option<SortOrder>,
    pub filter: Option<TaskField>,
    pub value: Option<String>,
}

impl fmt::Display for QueryOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Page: {}
Page Size: {}
Sort Field: {:?}
Sort Order: {:?}
Filter: {:?}
Include: {}",
            self.page.map_or("unset".to_string(), |v| v.to_string()),
            self.page_size
                .map_or("unset".to_string(), |v| v.to_string()),
            self.sort_field
                .map_or("unset".to_string(), |v| v.to_string()),
            self.sort_order
                .map_or("unset".to_string(), |v| v.to_string()),
            self.filter.map_or("unset".to_string(), |v| v.to_string()),
            self.value
                .as_ref()
                .map_or("unset".to_string(), |v| v.to_string())
        )
    }
}

impl Default for QueryOptions {
    fn default() -> Self {
        QueryOptions {
            page: Some(0usize),
            page_size: Some(5usize),
            sort_field: Some(TaskField::Created),
            sort_order: Some(SortOrder::Asc),
            filter: Some(TaskField::Status),
            value: Some(String::from("todo")),
        }
    }
}

pub trait IntoGetBy {
    fn into_get_by(self) -> GetBy;
}

impl IntoGetBy for GetBy {
    fn into_get_by(self) -> GetBy {
        self
    }
}

impl IntoGetBy for usize {
    fn into_get_by(self) -> GetBy {
        GetBy::ByIndex(self)
    }
}

impl IntoGetBy for Uuid {
    fn into_get_by(self) -> GetBy {
        GetBy::ByUuid(self)
    }
}

pub fn get_task_index(tasks: &[Task], by: &GetBy) -> Option<usize> {
    match by {
        GetBy::ByIndex(index) => {
            if *index < tasks.len() {
                Some(*index)
            } else {
                None
            }
        }
        GetBy::Last => {
            if tasks.is_empty() {
                None
            } else {
                Some(tasks.len() - 1)
            }
        }
        GetBy::ByUuid(uuid) => tasks.iter().position(|x| x.get_id() == uuid),
    }
}

pub fn apply_query(tasks: &[Task], query: &QueryOptions) -> Vec<Task> {
    let mut tasks = tasks.to_vec();

    if let Some(filter) = query.filter
        && let Some(v) = &query.value
    {
        tasks.retain(|t| match filter {
            TaskField::Title => v == &t.title,
            TaskField::Priority => v.to_lowercase() == t.priority.to_string().to_lowercase(),
            TaskField::Created => {
                unreachable!("Created filter should be rejected before apply_query")
            }
            TaskField::Status => v.to_lowercase() == t.done.to_string().to_lowercase(),
        });
    };

    tasks.sort_by(|a, b| {
        let ord = match query.sort_field.unwrap_or(TaskField::Created) {
            TaskField::Title => a.title.cmp(&b.title),
            TaskField::Priority => a.priority.cmp(&b.priority),
            TaskField::Created => a.get_created_at().cmp(&b.get_created_at()),
            TaskField::Status => a.done.cmp(&b.done),
        };
        match query.sort_order.unwrap_or(SortOrder::Asc) {
            SortOrder::Asc => ord,
            SortOrder::Desc => ord.reverse(),
        }
    });

    let page = query.page.unwrap_or(0);
    let size = query.page_size.unwrap_or(usize::MAX);

    let start = page * size;
    tasks.into_iter().skip(start).take(size).collect()
}
