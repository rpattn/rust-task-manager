use rust_task_manager::commands::handle_command;
use rust_task_manager::config::Config;
use rust_task_manager::display::print_table;
use rust_task_manager::parser::get_args;
use rust_task_manager::tasks::JsonStore;
use rust_task_manager::tasks::taskstore::TaskStore;

fn main() {
    let mut config = Config::default();
    if let Err(e) = config.load_config() {
        eprintln!("Warning: could not load config: {e}");
        // continues with defaults
    }

    let mut manager = JsonStore::new(config.get_tasks_filepath());
    match manager.open() {
        Ok(()) => {
            println!("Fetched tasks from {}", config.tasks_filename);
        }
        Err(e) => {
            println!("Error fetching tasks from {}", config.tasks_filename);
            println!("{e}");
        }
    }

    let cli_args = get_args();

    let command_result = handle_command(&config, cli_args, &mut manager);

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

    if let Err(e) = manager.close() {
        println!("{e}");
    }

    if let Some(tasks) = result.tasks {
        print_table(&tasks);
    }
}
