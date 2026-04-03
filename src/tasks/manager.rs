use crate::{
    store::{load, save},
    tasks::Task,
};

#[derive(Debug)]
pub struct Manager {
    tasks: Vec<Task>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager { tasks: Vec::new() }
    }
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn remove(&mut self, by: impl IntoRemoveBy) {
        match by.into_remove_by() {
            RemoveBy::ByTask(task) => {
                if let Some(pos) = self.tasks.iter().position(|x| x == &task) {
                    self.tasks.remove(pos);
                }
            }
            RemoveBy::ByIndex(index) => {
                self.tasks.remove(index);
            }
            RemoveBy::Last => {
                self.tasks.pop();
            }
        }
    }
    pub fn get(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.tasks.get_mut(index)
    }
    pub fn load_tasks(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tasks_str = load(filename).unwrap_or_default();

        if tasks_str.is_empty() {
            return Ok(());
        }

        let tasks: Result<Vec<Task>, serde_json::Error> = serde_json::from_str(&tasks_str);

        match tasks {
            Ok(tasks) => {
                self.tasks = tasks;
                Ok(())
            }
            Err(_) => {
                println!("Error loading tasks!");
                Ok(())
            }
        }
    }
    pub fn get_all(&self) -> &Vec<Task> {
        &self.tasks
    }
    pub fn save_tasks(&self, filename: &str) -> Result<(), std::io::Error> {
        let tasks_str = serde_json::to_string_pretty(&self.tasks).unwrap();
        save(filename, &tasks_str)
    }
    pub fn list_tasks(&self) {
        let tasks_json =
            serde_json::to_string_pretty(self.get_all()).expect("Error getting tasks json string");
        println!("Tasks: {tasks_json}");
    }
    pub fn clear_all_tasks(&mut self) {
        self.tasks = Vec::new();
    }
}

pub enum RemoveBy {
    ByTask(Task),
    ByIndex(usize),
    Last,
}

pub trait IntoRemoveBy {
    fn into_remove_by(self) -> RemoveBy;
}

impl IntoRemoveBy for RemoveBy {
    fn into_remove_by(self) -> RemoveBy {
        self
    }
}

impl IntoRemoveBy for Task {
    fn into_remove_by(self) -> RemoveBy {
        RemoveBy::ByTask(self)
    }
}

impl IntoRemoveBy for usize {
    fn into_remove_by(self) -> RemoveBy {
        RemoveBy::ByIndex(self)
    }
}
