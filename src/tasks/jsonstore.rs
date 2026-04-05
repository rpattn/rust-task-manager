use crate::{
    store::{load, save},
    tasks::{
        Task,
        task::TaskEdit,
        taskstore::{
            IntoGetBy, QueryOptions, TaskStore, TaskStoreError, apply_query, get_task_index,
        },
    },
};

#[derive(Debug)]
pub struct JsonStore {
    tasks: Vec<Task>,
    filename: String,
    dirty: bool,
}

// Implementing custom error type for future use
// e.g. matching on IoError for retry, fallback
#[derive(Debug, thiserror::Error)]
pub enum JsonStoreError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

impl From<JsonStoreError> for TaskStoreError {
    fn from(e: JsonStoreError) -> Self {
        match e {
            JsonStoreError::IoError(e) => TaskStoreError::BackendError(Box::new(e)),
            JsonStoreError::ParseError(e) => TaskStoreError::BackendError(Box::new(e)),
        }
    }
}

impl JsonStore {
    pub fn new(filename: &str) -> Self {
        JsonStore {
            tasks: Vec::default(),
            filename: filename.into(),
            dirty: false,
        }
    }
    fn load_tasks(&mut self) -> Result<(), JsonStoreError> {
        match load(&self.filename) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e.into()),
            Ok(s) if s.is_empty() => return Ok(()),
            Ok(s) => {
                self.tasks = serde_json::from_str(&s)?;
                self.dirty = false;
            }
        }
        Ok(())
    }
    fn save_tasks(&self) -> Result<(), JsonStoreError> {
        let tasks_str = serde_json::to_string_pretty(&self.tasks)?;
        save(&self.filename, &tasks_str)?;
        Ok(())
    }
}

impl TaskStore for JsonStore {
    fn open(&mut self) -> Result<(), TaskStoreError> {
        self.load_tasks()?;
        Ok(())
    }
    fn get<B: IntoGetBy>(&self, by: B) -> Option<Task> {
        get_task_index(&self.tasks, &by.into_get_by())
            .and_then(|i| self.tasks.get(i))
            .cloned()
    }
    fn add(&mut self, task: Task) {
        self.tasks.push(task);
        self.dirty = true;
    }
    fn edit(&mut self, by: impl IntoGetBy, edit: TaskEdit) -> Result<(), TaskStoreError> {
        let id = by.into_get_by();
        let task_index =
            get_task_index(&self.tasks, &id).ok_or(TaskStoreError::TaskNotFound { id })?;
        self.tasks
            .get_mut(task_index)
            .expect("index from get_task_index should always be valid")
            .edit(edit);
        self.dirty = true;
        Ok(())
    }
    fn remove(&mut self, by: impl IntoGetBy) -> Result<(), TaskStoreError> {
        let id = by.into_get_by();
        if let Some(index) = get_task_index(&self.tasks, &id) {
            self.tasks.remove(index);
            self.dirty = true;
            Ok(())
        } else {
            Err(TaskStoreError::TaskNotFound { id })
        }
    }
    fn get_all(&self, query: Option<&QueryOptions>) -> Vec<Task> {
        let Some(query) = query else {
            return self.tasks.clone();
        };

        apply_query(&self.tasks, query)
    }
    fn clear_all_tasks(&mut self) {
        self.tasks.clear();
        self.dirty = true;
    }
    fn close(&mut self) -> Result<(), TaskStoreError> {
        if !self.dirty {
            return Ok(());
        }
        self.save_tasks()?;
        self.dirty = false;
        Ok(())
    }
}
