mod tasks;
mod store;

use tasks::Manager;

const TASKS_FILENAME: &'static str = "out/tasks.json";

fn main() {
    let mut manager = Manager::new();
    manager.load_tasks(TASKS_FILENAME).expect("Error loading tasks");

    println!("Existing tasks: {:?}", manager);

    manager.save_tasks(TASKS_FILENAME).expect("Error saving tasks")
}
