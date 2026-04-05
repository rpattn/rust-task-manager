use rust_task_manager::commands::{CommandOutcome, handle_command};
use rust_task_manager::display::print_table;
use rust_task_manager::parser::get_args;
use rust_task_manager::tasks::Manager;
use rust_task_manager::tasks::taskstore::TaskStore;

const TASKS_FILENAME: &str = "out/tasks.json";

fn main() {
    let mut manager = Manager::new(TASKS_FILENAME);
    match manager.open() {
        Ok(()) => {}
        Err(e) => {
            println!("{e}");
        }
    }

    let cli_args = get_args();

    let command_result = handle_command(cli_args, &mut manager);

    let result = match command_result {
        Ok(r) => r,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    if let Some(message) = result.message {
        println!("{message}");
    }

    if let CommandOutcome::Mutated = result.outcome
        && let Err(e) = manager.close()
    {
        println!("{e}");
    }

    if let Some(tasks) = result.tasks {
        print_table(&tasks);
    }
}
