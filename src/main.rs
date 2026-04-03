mod parser;
mod store;
mod tasks;

use tasks::Manager;
use tasks::Task;
use tasks::manager::RemoveBy;

use parser::Command;
use parser::get_args;

const TASKS_FILENAME: &str = "out/tasks.json";

fn main() {
    let mut manager = Manager::new();
    manager
        .load_tasks(TASKS_FILENAME)
        .expect("Error loading tasks");

    let cli_args = get_args();

    let mut save_tasks = true;

    match cli_args.command {
        Some(Command::Get { id }) => {
            save_tasks = false;
            if let Some(taskid) = id {
                let task = manager.get(taskid);
                if let Some(task) = task {
                    println!("Task: {task}");
                } else {
                    println!("No task found for id: {taskid}");
                }
            } else {
                println!("Supply a task id");
            }
        }
        Some(Command::Add { name }) => {
            let mut task = Task::default();
            task.title = name;
            println!("Adding: {}", task);
            manager.add(task);
            println!("Added")
        }
        Some(Command::Remove { id, last }) => {
            if let Some(taskid) = id {
                manager.remove(RemoveBy::ByIndex(taskid));
                println!("Removed task with id: {taskid}")
            } else if last {
                manager.remove(RemoveBy::Last);
                println!("Removed the last task in the list")
            } else {
                println!("Supply a task id");
            }
        }
        Some(Command::Clear { force }) => {
            if force {
                manager.clear_all_tasks();
                println!("Cleared");
            } else {
                println!("Use --force to remove ALL tasks, this cannot be undone!!");
            }
        }
        Some(Command::Complete { id }) => {
            if let Some(taskid) = id {
                let task = manager.get_mut(taskid);
                if let Some(task) = task {
                    task.mark_complete();
                    println!("Task completed: {task}");
                } else {
                    println!("No task found for id: {:?}", id);
                }
            } else {
                println!("Supply a task id");
            }
        }
        None => {
            save_tasks = false;
            manager.list_tasks();
        }
    }

    if !save_tasks {
        return;
    }

    manager
        .save_tasks(TASKS_FILENAME)
        .expect("Error saving tasks");
}
