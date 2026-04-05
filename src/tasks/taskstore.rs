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
    fn get<B: IntoGetBy>(&self, by: B) -> Option<&Task>;
    fn add(&mut self, task: Task);
    fn edit(&mut self, by: impl IntoGetBy, edit: TaskEdit) -> Result<(), TaskStoreError>;
    fn remove(&mut self, by: impl IntoGetBy) -> Result<(), TaskStoreError>;
    fn open(&mut self) -> Result<(), TaskStoreError>;
    fn get_all(&self) -> &[Task];
    fn close(&mut self) -> Result<(), TaskStoreError>;
    fn clear_all_tasks(&mut self);
}

pub enum GetBy {
    ByIndex(usize),
    ByUuid(Uuid),
    Last,
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
