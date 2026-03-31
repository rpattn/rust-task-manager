mod tasks;
mod store;

use tasks::Manager;
use tasks::Task;

const TASKS_FILENAME: &'static str = "out/tasks.json";

fn main() {
    let mut manager = Manager::new();
    manager.load_tasks(TASKS_FILENAME).expect("Error loading tasks");

    println!("Existing tasks: {:?}", manager);

    let task = Task::default();

    manager.add_task(task);

    manager.save_tasks(TASKS_FILENAME).expect("Error saving tasks")
}
