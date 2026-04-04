use rust_task_manager::commands::{CommandOutcome, handle_command};
use rust_task_manager::display::print_table;
use rust_task_manager::parser::get_args;
use rust_task_manager::tasks::Manager;

const TASKS_FILENAME: &str = "out/tasks.json";

fn main() {
    let mut manager = Manager::default();
    match manager.load_tasks(TASKS_FILENAME) {
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
        && let Err(e) = manager.save_tasks(TASKS_FILENAME)
    {
        println!("{e}");
    }

    if let Some(tasks) = result.tasks {
        print_table(&tasks);
    }
}
