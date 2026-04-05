use crate::tasks::{
    Task,
    task::TaskEdit,
    taskstore::{IntoGetBy, QueryOptions, TaskStore, TaskStoreError, apply_query, get_task_index},
};

#[derive(Debug, Default)]
pub struct BasicStore {
    tasks: Vec<Task>,
}

impl TaskStore for BasicStore {
    fn open(&mut self) -> Result<(), TaskStoreError> {
        Ok(())
    }
    fn get<B: IntoGetBy>(&self, by: B) -> Option<Task> {
        get_task_index(&self.tasks, by).and_then(|i| self.tasks.get(i)).cloned()
    }
    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    fn edit(&mut self, by: impl IntoGetBy, edit: TaskEdit) -> Result<(), TaskStoreError> {
        let task_index = get_task_index(&self.tasks, by).ok_or(TaskStoreError::TaskNotFound)?;
        self.tasks
            .get_mut(task_index)
            .ok_or(TaskStoreError::TaskNotFound)?
            .edit(edit);
        Ok(())
    }
    fn remove(&mut self, by: impl IntoGetBy) -> Result<(), TaskStoreError> {
        if let Some(index) = get_task_index(&self.tasks, by) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(TaskStoreError::TaskNotFound)
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
    }
    fn close(&mut self) -> Result<(), TaskStoreError> {
        Ok(())
    }
}
