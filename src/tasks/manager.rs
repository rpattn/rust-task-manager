use crate::{store::{load, save}, tasks::Task};

#[derive(Debug)]
pub struct Manager {
   tasks: Vec<Task>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            tasks: Vec::new(),
        }
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn load_tasks(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tasks_str = load(filename).unwrap_or_default();

        if tasks_str.is_empty() {
            return Ok(());
        }

        let tasks: Vec<Task> = serde_json::from_str(&tasks_str)?;

        self.tasks = tasks;
        Ok(())
    }
    pub fn save_tasks(&self, filename: &str) -> Result<(), std::io::Error> {
        let tasks_str = serde_json::to_string_pretty(&self.tasks).unwrap();
        save(filename, &tasks_str)
    }

}
