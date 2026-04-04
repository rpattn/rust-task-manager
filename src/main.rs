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

enum CommandOutcome {
    Mutated,
    ReadOnly,
}

fn handle_command(args: Cli, manager: &mut Manager) -> Result<CommandOutcome, ManagerError> {
    match args.command {
        Some(Command::Get { id }) => {
            if let Some(taskid) = id {
                let task = manager.get(taskid);
                if let Some(task) = task {
                    println!("Task: {task}");
                    Ok(CommandOutcome::ReadOnly)
                } else {
                    Err(ManagerError::TaskNotFound)
                }
            } else {
                println!("Supply a task id or list index");
                println!("Tasks count is {}", manager.get_all().len());
                Ok(CommandOutcome::ReadOnly)
            }
        }
        Some(Command::Add { name, priority }) => {
            let mut task = Task::default();

            task.title = name;
            if let Some(priority) = priority {
                task.priority = priority;
            }

            println!("Adding: {}", task);
            manager.add(task);
            Ok(CommandOutcome::Mutated)
        }
        Some(Command::Remove { id, last }) => {
            if let Some(taskid) = id {
                manager.remove(taskid)?;
                println!("Removed task with id: {:?}", &taskid);
                Ok(CommandOutcome::Mutated)
            } else if last {
                manager.remove(GetBy::Last)?;
                println!("Removed the last task in the list");
                Ok(CommandOutcome::Mutated)
            } else {
                println!("Supply a task id");
                Ok(CommandOutcome::ReadOnly)
            }
        }
        Some(Command::Clear { force }) => {
            if force {
                println!("Cleared all tasks!");
                manager.clear_all_tasks().map(|_| CommandOutcome::Mutated)
            } else {
                println!("Use --force to remove ALL tasks, this cannot be undone!!");
                Ok(CommandOutcome::ReadOnly)
            }
        }
        Some(Command::Complete { id }) => {
            if let Some(taskid) = id {
                let task = manager.get_mut(taskid);
                if let Some(task) = task {
                    task.mark_complete();
                    println!("Task completed: {task}");
                    Ok(CommandOutcome::Mutated)
                } else {
                    Err(ManagerError::TaskNotFound)
                }
            } else {
                println!("Supply a task id");
                Ok(CommandOutcome::ReadOnly)
            }
        }
        None => {
            manager.list_tasks();
            Ok(CommandOutcome::ReadOnly) // list tasks  by default
        }
    }
}

fn print_error_ln(e: ManagerError) {
    println!("Error: {e}");
}

fn main() {
    let mut manager = Manager::default();
    match manager.load_tasks(TASKS_FILENAME) {
        Ok(()) => {}
        Err(e) => {
            print_error_ln(e);
        }
    }

    let cli_args = get_args();

    let save_tasks = handle_command(cli_args, &mut manager);

    match save_tasks {
        Ok(save_tasks) => match save_tasks {
            CommandOutcome::Mutated => match manager.save_tasks(TASKS_FILENAME) {
                Ok(()) => {}
                Err(e) => print_error_ln(e),
            },
            CommandOutcome::ReadOnly => {},
        },
        Err(e) => print_error_ln(e),
    }
}
