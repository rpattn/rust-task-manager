mod tasks;
mod store;

use tasks::Manager;
use tasks::Task;

use crate::tasks::manager::RemoveBy;

const TASKS_FILENAME: &'static str = "out/tasks.json";

fn main() {
    let mut manager = Manager::new();
    manager.load_tasks(TASKS_FILENAME).expect("Error loading tasks");

    let task = Task::default();

    manager.add(task.clone());
    manager.add(task.clone());
    manager.add(task.clone());

    if let Some(task) = manager.get(1).cloned() {
        manager.remove(task);
    };

    manager.remove(RemoveBy::Last);

    println!("Tasks: {:?}", manager);

    manager.save_tasks(TASKS_FILENAME).expect("Error saving tasks")
}
