use crate::{
    store::{load, save},
    tasks::{
        Task,
        taskstore::{GetBy, IntoGetBy, TaskStore},
    },
};

#[derive(Debug, Default)]
pub struct Manager {
    tasks: Vec<Task>,
    filename: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    #[error("Task not found")]
    TaskNotFound,
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

impl Manager {
    pub fn new(filename: &str) -> Self {
        Manager {
            tasks: Vec::default(),
            filename: filename.into(),
        }
    }
    fn get_index(&self, by: impl IntoGetBy) -> Option<usize> {
        match by.into_get_by() {
            GetBy::ByIndex(index) => {
                if index < self.tasks.len() {
                    Some(index)
                } else {
                    None
                }
            }
            GetBy::Last => {
                if self.tasks.is_empty() {
                    None
                } else {
                    Some(self.tasks.len() - 1)
                }
            }
            GetBy::ByUuid(uuid) => self.tasks.iter().position(|x| x.get_id() == &uuid),
        }
    }
    fn load_tasks(&mut self) -> Result<(), ManagerError> {
        match load(&self.filename) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e.into()),
            Ok(s) if s.is_empty() => return Ok(()),
            Ok(s) => self.tasks = serde_json::from_str(&s)?,
        }
        Ok(())
    }
    fn save_tasks(&self) -> Result<(), ManagerError> {
        let tasks_str = serde_json::to_string_pretty(&self.tasks)?;
        save(&self.filename, &tasks_str)?;
        Ok(())
    }
}

impl TaskStore for Manager {
    fn open(&mut self) -> Result<(), ManagerError> {
        self.load_tasks()
    }
    fn get<B: IntoGetBy>(&self, by: B) -> Option<&Task> {
        self.get_index(by).and_then(|i| self.tasks.get(i))
    }
    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    fn get_mut(&mut self, by: impl IntoGetBy) -> Option<&mut Task> {
        self.get_index(by).and_then(|i| self.tasks.get_mut(i))
    }
    fn remove(&mut self, by: impl IntoGetBy) -> Result<(), ManagerError> {
        if let Some(index) = self.get_index(by) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(ManagerError::TaskNotFound)
        }
    }
    fn get_all(&self) -> &[Task] {
        &self.tasks
    }
    fn clear_all_tasks(&mut self) {
        self.tasks.clear();
    }
    fn close(&self) -> Result<(), ManagerError> {
        self.save_tasks()
    }
}
