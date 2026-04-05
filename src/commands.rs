use crate::parser::{Cli, Command};
use crate::tasks::task::TaskEdit;
use crate::tasks::taskstore::{GetBy, TaskStore};
use crate::tasks::{ManagerError, Task};

pub struct CommandResult {
    pub tasks: Option<Vec<Task>>,
    pub outcome: CommandOutcome,
    pub message: Option<String>,
}

pub enum CommandOutcome {
    Mutated,
    ReadOnly,
}

pub fn handle_command<S: TaskStore>(
    args: Cli,
    manager: &mut S,
) -> Result<CommandResult, ManagerError> {
    match args.command {
        Some(Command::Get { id }) => {
            if let Some(taskid) = id {
                let task = manager.get(taskid);
                if let Some(task) = task {
                    Ok(CommandResult {
                        tasks: Some(vec![task.clone()]),
                        outcome: CommandOutcome::ReadOnly,
                        message: None,
                    })
                } else {
                    Err(ManagerError::TaskNotFound)
                }
            } else {
                Ok(CommandResult {
                    tasks: Some(manager.get_all().to_vec()),
                    outcome: CommandOutcome::ReadOnly,
                    message: Some("Supply a task id after get keyword. Tasks in list are: ".into()),
                })
            }
        }
        Some(Command::Add { name, priority }) => {
            let mut task = Task::default();

            task.title = name;
            if let Some(priority) = priority {
                task.priority = priority;
            }
            manager.add(task.clone());
            Ok(CommandResult {
                tasks: Some(vec![task]),
                outcome: CommandOutcome::Mutated,
                message: Some("Added task".into()),
            })
        }
        Some(Command::Edit {
            id,
            title,
            priority,
        }) => {
            let task = manager.get_mut(id).ok_or(ManagerError::TaskNotFound)?;
            task.edit(TaskEdit { title, priority });
            Ok(CommandResult {
                tasks: Some(vec![task.clone()]),
                outcome: CommandOutcome::Mutated,
                message: Some("Task updated".into()),
            })
        }
        Some(Command::Remove { id, last }) => {
            if let Some(taskid) = id {
                manager.remove(taskid)?;
                Ok(CommandResult {
                    tasks: None,
                    outcome: CommandOutcome::Mutated,
                    message: Some("Removed task".into()),
                })
            } else if last {
                manager.remove(GetBy::Last)?;
                Ok(CommandResult {
                    tasks: None,
                    outcome: CommandOutcome::Mutated,
                    message: Some("Removed the last task in the list".into()),
                })
            } else {
                Ok(CommandResult {
                    tasks: None,
                    outcome: CommandOutcome::ReadOnly,
                    message: Some("Supply a task id".into()),
                })
            }
        }
        Some(Command::Clear { force }) => {
            if force {
                manager.clear_all_tasks();
                Ok(CommandResult {
                    tasks: None,
                    outcome: CommandOutcome::Mutated,
                    message: Some("Cleared all tasks!".into()),
                })
            } else {
                Ok(CommandResult {
                    tasks: Some(manager.get_all().to_vec()),
                    outcome: CommandOutcome::ReadOnly,
                    message: Some(
                        "Use --force to remove ALL tasks, this cannot be undone!!".into(),
                    ),
                })
            }
        }
        Some(Command::Complete { id }) => {
            let task = manager.get_mut(id).ok_or(ManagerError::TaskNotFound)?;
            task.mark_complete();
            Ok(CommandResult {
                tasks: Some(vec![task.clone()]),
                outcome: CommandOutcome::Mutated,
                message: Some("Task completed".into()),
            })
        }
        None => Ok(CommandResult {
            tasks: Some(manager.get_all().to_vec()),
            outcome: CommandOutcome::ReadOnly,
            message: None,
        }),
    }
}
