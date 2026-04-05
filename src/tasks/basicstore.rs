use crate::tasks::{
    Task,
    task::TaskEdit,
    taskstore::{GetBy, IntoGetBy, TaskStore, TaskStoreError},
};

#[derive(Debug, Default)]
pub struct BasicStore {
    tasks: Vec<Task>,
}

impl BasicStore {
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
}

impl TaskStore for BasicStore {
    fn open(&mut self) -> Result<(), TaskStoreError> {
        Ok(())
    }
    fn get<B: IntoGetBy>(&self, by: B) -> Option<&Task> {
        self.get_index(by).and_then(|i| self.tasks.get(i))
    }
    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    fn edit(&mut self, by: impl IntoGetBy, edit: TaskEdit) -> Result<(), TaskStoreError> {
        let task_index = self.get_index(by).ok_or(TaskStoreError::TaskNotFound)?;
        self.tasks
            .get_mut(task_index)
            .ok_or(TaskStoreError::TaskNotFound)?
            .edit(edit);
        Ok(())
    }
    fn remove(&mut self, by: impl IntoGetBy) -> Result<(), TaskStoreError> {
        if let Some(index) = self.get_index(by) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(TaskStoreError::TaskNotFound)
        }
    }
    fn get_all(&self) -> &[Task] {
        &self.tasks
    }
    fn clear_all_tasks(&mut self) {
        self.tasks.clear();
    }
    fn close(&mut self) -> Result<(), TaskStoreError> {
        Ok(())
    }
}
