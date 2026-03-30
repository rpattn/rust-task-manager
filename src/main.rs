mod tasks;

use tasks::Task;

fn main() {
    let task = Task::default();

    println!("Hello, world!");
    println!("Task {task}");
}
