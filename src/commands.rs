use crate::parser::{Cli, Command};
use crate::tasks::Task;
use crate::tasks::task::{Status, TaskEdit};
use crate::tasks::taskstore::{GetBy, TaskStore, TaskStoreError};

pub struct CommandResult {
    pub tasks: Option<Vec<Task>>,
    pub message: Option<String>,
}

pub fn handle_command<S: TaskStore>(
    args: Cli,
    manager: &mut S,
) -> Result<CommandResult, TaskStoreError> {
    match args.command {
        Some(Command::Get { id }) => {
            if let Some(taskid) = id {
                let task = manager.get(taskid);
                if let Some(task) = task {
                    Ok(CommandResult {
                        tasks: Some(vec![task.clone()]),
                        message: None,
                    })
                } else {
                    Err(TaskStoreError::TaskNotFound)
                }
            } else {
                Ok(CommandResult {
                    tasks: Some(manager.get_all().to_vec()),
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
                message: Some("Added task".into()),
            })
        }
        Some(Command::Edit {
            id,
            title,
            priority,
            status,
        }) => {
            manager.edit(
                id,
                TaskEdit {
                    title,
                    priority,
                    status,
                },
            )?;
            let edited_task = manager.get(id).ok_or(TaskStoreError::TaskNotFound)?.clone();
            Ok(CommandResult {
                tasks: Some(vec![edited_task]),
                message: Some("Task updated".into()),
            })
        }
        Some(Command::Remove { id, last }) => {
            if let Some(taskid) = id {
                manager.remove(taskid)?;
                Ok(CommandResult {
                    tasks: None,
                    message: Some("Removed task".into()),
                })
            } else if last {
                manager.remove(GetBy::Last)?;
                Ok(CommandResult {
                    tasks: None,
                    message: Some("Removed the last task in the list".into()),
                })
            } else {
                Ok(CommandResult {
                    tasks: None,
                    message: Some("Supply a task id".into()),
                })
            }
        }
        Some(Command::Clear { force }) => {
            if force {
                manager.clear_all_tasks();
                Ok(CommandResult {
                    tasks: None,
                    message: Some("Cleared all tasks!".into()),
                })
            } else {
                Ok(CommandResult {
                    tasks: Some(manager.get_all().to_vec()),
                    message: Some(
                        "Use --force to remove ALL tasks, this cannot be undone!!".into(),
                    ),
                })
            }
        }
        Some(Command::Complete { id }) => {
            manager.edit(
                id,
                TaskEdit {
                    title: None,
                    priority: None,
                    status: Some(Status::Complete),
                },
            )?;
            let edited_task = manager.get(id).ok_or(TaskStoreError::TaskNotFound)?.clone();
            Ok(CommandResult {
                tasks: Some(vec![edited_task]),
                message: Some("Task completed".into()),
            })
        }
        None => Ok(CommandResult {
            tasks: Some(manager.get_all().to_vec()),
            message: None,
        }),
    }
}
