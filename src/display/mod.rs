use comfy_table::Table;
use comfy_table::{Cell, Color};

use crate::tasks::Task;
use crate::tasks::task::{Priority, Status};

pub fn status_to_cell(status: &Status) -> Cell {
    match status {
        Status::Todo => Cell::new("⬜ Todo").fg(Color::Yellow),
        Status::Complete => Cell::new("✅ Done").fg(Color::Green),
    }
}

pub fn priority_to_cell(priority: &Priority) -> Cell {
    match priority {
        Priority::Low => Cell::new("Low").fg(Color::Green),
        Priority::Medium => Cell::new("Medium").fg(Color::Yellow),
        Priority::High => Cell::new("High").fg(Color::Red),
    }
}

pub fn print_table(tasks: &[Task]) {
    let mut table = Table::new();
    table.set_header(vec!["Status", "Title", "Priority", "Index", "ID"]);

    for (i, task) in tasks.iter().enumerate() {
        table.add_row(vec![
            status_to_cell(&task.done), // These helpers are in task.rs
            task.title.clone().into(),
            priority_to_cell(&task.priority),
            i.to_string().into(),
            task.get_id().to_string().into(),
        ]);
    }
    println!("{table}");
}
