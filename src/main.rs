mod tasks;
mod store;

use tasks::Manager;
use tasks::manager::RemoveBy;
use tasks::Task;

const TASKS_FILENAME: &'static str = "out/tasks.json";

fn main() {
    let mut manager = Manager::new();
    manager.load_tasks(TASKS_FILENAME).expect("Error loading tasks");

    // Add a couple of tasks
    manager.add(Task::default());
    manager.add(Task::default());

    // Add and remove by task ref
    manager.add(Task::default());
    if let Some(task) = manager.get(0) {
        manager.remove(task.clone());
    }

    // Rename and complete the first
    if let Some(task) = manager.get_mut(0) {
        task.title = String::from("Buy groceries");
        task.priority = tasks::task::Priority::High;
        task.mark_complete();
    }

    // Rename the second
    if let Some(task) = manager.get_mut(1) {
        task.title = String::from("Walk the dog");
    }

    // Add and remove by last
    manager.add(Task::default());
    manager.remove(RemoveBy::Last);

    let tasks_json = serde_json::to_string_pretty(manager.get_all()).expect("Error getting tasks json string");
    println!("Tasks: {tasks_json}");

    manager.save_tasks(TASKS_FILENAME).expect("Error saving tasks");
}
