use crate::parser::{Cli, Command, IdArg};
use crate::tasks::Task;
use crate::tasks::task::{Status, TaskEdit};
use crate::tasks::taskstore::{
    GetBy, QueryOptions, SortOrder, TaskField, TaskStore, TaskStoreError,
};

pub struct CommandResult {
    pub tasks: Option<Vec<Task>>,
    pub message: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("not enough arguments passed to {command}")]
    NotEnoughArgs { command: String },
    #[error("query error: {reason}")]
    QueryError { reason: String },
    #[error("no task found for id: {id}")]
    TaskNotFound { id: IdArg },
    #[error(transparent)]
    Store(#[from] TaskStoreError),
}

pub fn handle_command<S: TaskStore>(
    args: Cli,
    manager: &mut S,
) -> Result<CommandResult, CommandError> {
    match args.command {
        Some(Command::List {
            page,
            size,
            sort,
            order,
            filter,
            value,
        }) => {
            if matches!(filter, Some(TaskField::Created)) {
                return Err(CommandError::QueryError {
                    reason: "unsupported filter field".into(),
                });
            }
            let query = match (page, size, sort, order, filter, value.clone()) {
                (None, None, None, None, None, None) => None,
                _ => Some(QueryOptions {
                    page: page.unwrap_or(0),
                    page_size: size.unwrap_or(10),
                    sort_field: sort.unwrap_or(TaskField::Created),
                    sort_order: order.unwrap_or(SortOrder::Asc),
                    filter,
                    value,
                }),
            };
            Ok(CommandResult {
                tasks: Some(manager.get_all(query.as_ref())),
                message: None,
            })
        }
        Some(Command::Get { id }) => {
            let id = id.ok_or(CommandError::NotEnoughArgs { command: "Get".into()})?;
            // maybe just return the task from add, thinking about keeping for a SQL db where add
            // may fail and we may not have added the task to the db
            let task = manager.get(id).ok_or(CommandError::TaskNotFound { id })?;
            Ok(CommandResult {
                tasks: Some(vec![task]),
                message: None,
            })
        }
        Some(Command::Add { name, priority }) => {
            let mut task = Task::default();

            task.title = name;
            if let Some(priority) = priority {
                task.priority = priority;
            }
            let id = *task.get_id();
            manager.add(task);
            let task = manager.get(id).ok_or(CommandError::TaskNotFound {
                id: IdArg::Uuid { uuid: id },
            })?;
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
            let edited_task = manager.get(id).ok_or(CommandError::TaskNotFound { id })?;
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
                Err(CommandError::NotEnoughArgs {
                    command: "Remove".into(),
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
                    tasks: Some(manager.get_all(None)),
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
            let edited_task = manager.get(id).ok_or(CommandError::TaskNotFound { id })?;
            Ok(CommandResult {
                tasks: Some(vec![edited_task]),
                message: Some("Task completed".into()),
            })
        }
        None => Ok(CommandResult {
            tasks: Some(manager.get_all(None)),
            message: None,
        }),
    }
}
