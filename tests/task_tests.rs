// tests/task_tests.rs
use rust_task_manager::tasks::task::{Priority, Status};
use rust_task_manager::tasks::Task;
use rust_task_manager::tasks::task::TaskEdit;

// --- defaults ---

#[test]
fn default_task_has_todo_status() {
    let task = Task::default();
    assert_eq!(task.done, Status::Todo);
}

#[test]
fn default_task_has_low_priority() {
    let task = Task::default();
    assert_eq!(task.priority, Priority::Low);
}

#[test]
fn default_task_has_unique_ids() {
    let a = Task::default();
    let b = Task::default();
    assert_ne!(a.get_id(), b.get_id());
}

// --- mark_complete ---

#[test]
fn mark_complete_sets_status_to_complete() {
    let mut task = Task::default();
    task.mark_complete();
    assert_eq!(task.done, Status::Complete);
}

#[test]
fn mark_complete_is_idempotent() {
    let mut task = Task::default();
    task.mark_complete();
    task.mark_complete();
    assert_eq!(task.done, Status::Complete);
}

// --- edit ---

#[test]
fn edit_updates_title() {
    let mut task = Task::default();
    task.edit(TaskEdit { title: Some("new title".into()), priority: None });
    assert_eq!(task.title, "new title");
}

#[test]
fn edit_updates_priority() {
    let mut task = Task::default();
    task.edit(TaskEdit { title: None, priority: Some(Priority::High) });
    assert_eq!(task.priority, Priority::High);
}

#[test]
fn edit_with_all_none_is_no_op() {
    let mut task = Task::default();
    let original_title = task.title.clone();
    task.edit(TaskEdit { title: None, priority: None });
    assert_eq!(task.title, original_title);
    assert_eq!(task.priority, Priority::Low);
}

#[test]
fn edit_partial_only_changes_provided_fields() {
    let mut task = Task::default();
    task.priority = Priority::High;
    task.edit(TaskEdit { title: Some("updated".into()), priority: None });
    assert_eq!(task.title, "updated");
    assert_eq!(task.priority, Priority::High); // unchanged
}
