mod tasks;
mod store;

use tasks::Manager;
use tasks::manager::RemoveBy;
use tasks::Task;

const TASKS_FILENAME: &'static str = "out/tasks.json";

fn main() {
    let mut manager = Manager::new();
    manager.load_tasks(TASKS_FILENAME).expect("Error loading tasks");

    let task = Task::default();

    manager.add(task.clone());
    if let Some(task) = manager.get(1).cloned() {
        manager.remove(task);
    };

    manager.add(task.clone());
    manager.remove(RemoveBy::Last);

    println!("Tasks: {:?}", manager);

    manager.save_tasks(TASKS_FILENAME).expect("Error saving tasks")
}
