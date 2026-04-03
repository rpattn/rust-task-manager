use uuid::Uuid;

use crate::{
    parser::IdArg,
    store::{load, save},
    tasks::Task,
};

#[derive(Debug)]
pub struct Manager {
    tasks: Vec<Task>,
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
    pub fn new() -> Manager {
        Manager { tasks: Vec::new() }
    }
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn get(&self, by: impl IntoGetBy) -> Option<&Task> {
        match by.into_get_by() {
            GetBy::ByIndex(index) => self.tasks.get(index),
            GetBy::Last => self.tasks.get(self.tasks.len() - 1),
            GetBy::ByUuid(uuid) => self.tasks.iter().find(|x| x.get_id() == &uuid),
            GetBy::ByIdArg(id_arg) => match id_arg {
                IdArg::Index { index } => self.tasks.get(index),
                IdArg::Uuid { uuid } => self.tasks.iter().find(|x| x.get_id() == &uuid),
            },
        }
    }
    pub fn remove(&mut self, by: impl IntoGetBy) -> Result<(), ManagerError> {
        if let Some(task) = self.get(by) {
            if let Some(pos) = self.tasks.iter().position(|x| x == task) {
                self.tasks.remove(pos);
                Ok(())
            } else {
                Err(ManagerError::TaskNotFound)
            }
        } else {
            Err(ManagerError::TaskNotFound)
        }
    }
    pub fn get_mut(&mut self, by: impl IntoGetBy) -> Option<&mut Task> {
        if let Some(task) = self.get(by) {
            if let Some(index) = self.tasks.iter().position(|x| x == task) {
                self.tasks.get_mut(index)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn load_tasks(&mut self, filename: &str) -> Result<(), ManagerError> {
        let tasks_str = load(filename)?;
        if tasks_str.is_empty() {
            println!("Tasks file is empty, starting new list");
            return Ok(());
        }

        self.tasks = serde_json::from_str::<Vec<Task>>(&tasks_str)?;
        Ok(())
    }
    pub fn get_all(&self) -> &Vec<Task> {
        &self.tasks
    }
    pub fn save_tasks(&self, filename: &str) -> Result<(), ManagerError> {
        let tasks_str = serde_json::to_string_pretty(&self.tasks)?;
        save(filename, &tasks_str)?;
        Ok(())
    }
    pub fn list_tasks(&self) -> Result<(), ManagerError> {
        let tasks_json = serde_json::to_string_pretty(self.get_all())?;
        println!("Tasks: {tasks_json}");
        Ok(())
    }
    pub fn clear_all_tasks(&mut self) {
        self.tasks = Vec::new();
    }
}

pub enum GetBy {
    ByIndex(usize),
    ByUuid(Uuid),
    ByIdArg(IdArg),
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
impl IntoGetBy for IdArg {
    fn into_get_by(self) -> GetBy {
        GetBy::ByIdArg(self)
    }
}
