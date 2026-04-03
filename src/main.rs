mod parser;
mod store;
mod tasks;

use tasks::Manager;
use tasks::Task;

use parser::Command;
use parser::get_args;

use tasks::GetBy;

use parser::Cli;
use tasks::ManagerError;

const TASKS_FILENAME: &str = "out/tasks.json";

fn handle_command(args: Cli, manager: &mut Manager) -> Result<bool, ManagerError> {
    match args.command {
        Some(Command::Get { id }) => {
            if let Some(taskid) = id {
                let task = manager.get(taskid);
                if let Some(task) = task {
                    println!("Task: {task}");
                    Ok(false)
                } else {
                    println!("No task found for that id");
                    Err(ManagerError::TaskNotFound)
                }
            } else {
                println!("Supply a task id or list index");
                println!("Tasks count is {}", manager.get_all().len().to_string());
                Ok(false)
            }
        }
        Some(Command::Add { name }) => {
            let mut task = Task::default();
            task.title = name;
            println!("Adding: {}", task);
            manager.add(task);
            println!("Added");
            Ok(true)
        }
        Some(Command::Remove { id, last }) => {
            if let Some(taskid) = id {
                manager.remove(taskid)?;
                println!("Removed task with id: {:?}", &taskid);
                Ok(true)
            } else if last {
                manager.remove(GetBy::Last)?;
                println!("Removed the last task in the list");
                Ok(true)
            } else {
                println!("Supply a task id");
                Ok(false)
            }
        }
        Some(Command::Clear { force }) => {
            if force {
                manager.clear_all_tasks();
                println!("Cleared");
                Ok(true)
            } else {
                println!("Use --force to remove ALL tasks, this cannot be undone!!");
                Ok(false)
            }
        }
        Some(Command::Complete { id }) => {
            if let Some(taskid) = id {
                let task = manager.get_mut(taskid);
                if let Some(task) = task {
                    task.mark_complete();
                    println!("Task completed: {task}");
                    Ok(true)
                } else {
                    println!("No task found for id: {:?}", id);
                    Ok(false)
                }
            } else {
                println!("Supply a task id");
                Ok(false)
            }
        }
        None => {
            match manager.list_tasks() {
                Ok(()) => Ok(false),
                Err(e) => Err(e)
            }
        }
    }
}

fn main() {
    let mut manager = Manager::new();
    match manager.load_tasks(TASKS_FILENAME) {
        Ok(()) => {}
        Err(e) => {
            println!("Error: {e}")
        }
    }

    let cli_args = get_args();

    let save_tasks = handle_command(cli_args, &mut manager);

    match save_tasks {
        Ok(save_tasks) => match save_tasks {
            true => match manager.save_tasks(TASKS_FILENAME) {
                Ok(()) => {}
                Err(e) => println!("Error: {e}"),
            },
            false => return,
        },
        Err(e) => {
            println!("Error: {e}")
        }
    }

    // let last = manager.get_mut(GetBy::Last);
    // if let Some(task) = last {
    //    task.title = String::from("This is the last task in the list");
    //    save_tasks = true;
    // }
}
