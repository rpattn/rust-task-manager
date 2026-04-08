use crate::config::{Config, ConfigError, ConfigFields};
use crate::parser::{Command, IdArg};
use crate::tasks::Task;
use crate::tasks::task::{Status, TaskEdit};
use crate::tasks::taskstore::{GetBy, QueryOptions, TaskField, TaskStore, TaskStoreError};

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
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}

pub fn handle_command<S: TaskStore>(
    config: &mut Config,
    command: Option<Command>,
    manager: &mut S,
) -> Result<CommandResult, CommandError> {
    match command {
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
            let query = QueryOptions {
                page: page.or(config.query_options.page),
                page_size: size.or(config.query_options.page_size),
                sort_field: sort.or(config.query_options.sort_field),
                sort_order: order.or(config.query_options.sort_order),
                filter: filter.or(config.query_options.filter),
                value: value.or(config.query_options.value.clone()),
            };
            Ok(CommandResult {
                tasks: Some(manager.get_all(Some(&query))),
                message: None,
            })
        }
        Some(Command::Config {
            tasks_filename,
            config_filename,
            page,
            size,
            sort,
            order,
            filter,
            value,
            clear_filter,
        }) => {
            if matches!(
                (
                    &tasks_filename,
                    &config_filename,
                    &page,
                    &size,
                    &sort,
                    &order,
                    &filter,
                    &value,
                    &clear_filter,
                ),
                (None, None, None, None, None, None, None, None, false)
            ) {
                return Err(CommandError::NotEnoughArgs {
                    command: "Config".into(),
                });
            }
            if clear_filter {
                config.clear_filter()?;
            }
            config.update_config(ConfigFields {
                tasks_filename,
                config_filename,
                query_options: QueryOptions {
                    page,
                    page_size: size,
                    sort_field: sort,
                    sort_order: order,
                    filter,
                    value,
                },
            })?;
            Ok(CommandResult {
                tasks: None,
                message: None,
            })
        }
        Some(Command::Info) => Ok(CommandResult {
            tasks: None,
            message: Some(config.to_string()),
        }),
        Some(Command::Get { id }) => {
            let id = id.ok_or(CommandError::NotEnoughArgs {
                command: "Get".into(),
            })?;
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
            if matches!((&title, &priority, &status), (None, None, None)) {
                return Err(CommandError::NotEnoughArgs {
                    command: "Edit".into(),
                });
            }
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
                message: Some("Marked task as completed".into()),
            })
        }
        None => Ok(CommandResult {
            tasks: Some(manager.get_all(None)), // list all by default, no query
            message: None,
        }),
    }
}
