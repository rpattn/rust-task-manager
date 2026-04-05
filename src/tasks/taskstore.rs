use uuid::Uuid;

use crate::tasks::{ManagerError, Task, task::TaskEdit};

pub trait TaskStore {
    fn get<B: IntoGetBy>(&self, by: B) -> Option<&Task>;
    fn add(&mut self, task: Task);
    fn edit(&mut self, by: impl IntoGetBy, edit: TaskEdit) -> Result<(), ManagerError>;
    fn remove(&mut self, by: impl IntoGetBy) -> Result<(), ManagerError>;
    fn open(&mut self) -> Result<(), ManagerError>;
    fn get_all(&self) -> &[Task];
    fn close(&self) -> Result<(), ManagerError>;
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
