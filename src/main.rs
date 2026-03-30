mod tasks;
mod store;

use tasks::Task;
use store::save;

use crate::store::load;

const TASKS_FILENAME: &'static str = "out/tasks.json";

fn main() {
    let task = Task::default();
    let task_json = serde_json::to_string(&task).unwrap();

    let tasks_str = load(TASKS_FILENAME).expect("Failed to load file");

    println!("Existing tasks: {tasks_str}");

    save(TASKS_FILENAME, &task_json).expect("Failed to save file");

    println!("Hello, world!");
    println!("Task {task}");
}
