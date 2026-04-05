use uuid::Uuid;

use crate::tasks::{Task, task::TaskEdit};

#[derive(Debug, thiserror::Error)]
pub enum TaskStoreError {
    #[error("task not found")]
    TaskNotFound,
    #[error("backend error: {0}")]
    BackendError(#[from] Box<dyn std::error::Error>),
}

pub trait TaskStore {
    fn get<B: IntoGetBy>(&self, by: B) -> Option<Task>;
    fn add(&mut self, task: Task);
    fn edit(&mut self, by: impl IntoGetBy, edit: TaskEdit) -> Result<(), TaskStoreError>;
    fn remove(&mut self, by: impl IntoGetBy) -> Result<(), TaskStoreError>;
    fn open(&mut self) -> Result<(), TaskStoreError>;
    fn get_all(&self, page: Option<&QueryOptions>) -> Vec<Task>;
    fn close(&mut self) -> Result<(), TaskStoreError>;
    fn clear_all_tasks(&mut self);
}

pub enum GetBy {
    ByIndex(usize),
    ByUuid(Uuid),
    Last,
}

#[derive(Clone, Debug, Copy, clap::ValueEnum)]
pub enum TaskField {
    Title,
    Priority,
    Created,
    Status,
}

#[derive(Clone, Debug, Copy, clap::ValueEnum)]
pub enum SortOrder {
    Asc,
    Desc,
}

pub struct QueryOptions {
    pub page: usize,
    pub page_size: usize,
    pub sort_field: TaskField,
    pub sort_order: SortOrder,
    pub filter: Option<TaskField>,
    pub value: Option<String>,
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

pub fn get_task_index(tasks: &[Task], by: impl IntoGetBy) -> Option<usize> {
    match by.into_get_by() {
        GetBy::ByIndex(index) => {
            if index < tasks.len() {
                Some(index)
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
        GetBy::ByUuid(uuid) => tasks.iter().position(|x| x.get_id() == &uuid),
    }
}

pub fn apply_query(tasks: &[Task], query: &QueryOptions) -> Vec<Task> {
    let mut tasks = tasks.to_vec();

    if let Some(filter) = query.filter
        && let Some(v) = &query.value {
        tasks.retain(|t| {
            match filter {
                TaskField::Title => v == &t.title,
                TaskField::Priority => v.to_lowercase() == t.priority.to_string().to_lowercase(),
                TaskField::Created => v == &t.get_created_at().to_string(), //consider removing
                TaskField::Status => v.to_lowercase() == t.done.to_string().to_lowercase(),
            }
        });
    };

    tasks.sort_by(|a, b| {
        let ord = match query.sort_field {
            TaskField::Title => a.title.cmp(&b.title),
            TaskField::Priority => a.priority.cmp(&b.priority),
            TaskField::Created => a.get_created_at().cmp(&b.get_created_at()),
            TaskField::Status => a.done.cmp(&b.done),
        };
        match query.sort_order {
            SortOrder::Asc => ord,
            SortOrder::Desc => ord.reverse(),
        }
    });

    let start = query.page * query.page_size;
    tasks
        .into_iter()
        .skip(start)
        .take(query.page_size)
        .collect()
}
