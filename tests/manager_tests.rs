// tests/manager_tests.rs
use rust_task_manager::tasks::task::Priority;
use rust_task_manager::tasks::taskstore::{GetBy, TaskStore};
use rust_task_manager::tasks::{Manager, Task};

fn make_task(title: &str) -> Task {
    let mut task = Task::default();
    task.title = title.into();
    task
}

fn manager_with_tasks(titles: &[&str]) -> Manager {
    let mut manager = Manager::default();
    for title in titles {
        manager.add(make_task(title));
    }
    manager
}

// --- add ---

#[test]
fn add_single_task_increases_count() {
    let mut manager = Manager::default();
    manager.add(Task::default());
    assert_eq!(manager.get_all().len(), 1);
}

#[test]
fn add_multiple_tasks_preserves_order() {
    let manager = manager_with_tasks(&["first", "second", "third"]);
    let tasks = manager.get_all();
    assert_eq!(tasks[0].title, "first");
    assert_eq!(tasks[1].title, "second");
    assert_eq!(tasks[2].title, "third");
}

// --- get by index ---

#[test]
fn get_by_index_returns_correct_task() {
    let manager = manager_with_tasks(&["a", "b", "c"]);
    assert_eq!(manager.get(1usize).unwrap().title, "b");
}

#[test]
fn get_by_index_out_of_bounds_returns_none() {
    let manager = manager_with_tasks(&["only"]);
    assert!(manager.get(99usize).is_none());
}

#[test]
fn get_by_index_on_empty_manager_returns_none() {
    let manager = Manager::default();
    assert!(manager.get(0usize).is_none());
}

// --- get by uuid ---

#[test]
fn get_by_uuid_returns_correct_task() {
    let mut manager = Manager::default();
    let task = make_task("find me");
    let id = *task.get_id();
    manager.add(task);

    let found = manager.get(id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().title, "find me");
}

#[test]
fn get_by_unknown_uuid_returns_none() {
    let manager = manager_with_tasks(&["a"]);
    let random_id = uuid::Uuid::new_v4();
    assert!(manager.get(random_id).is_none());
}

// --- get last ---

#[test]
fn get_last_returns_final_task() {
    let manager = manager_with_tasks(&["first", "last"]);
    assert_eq!(manager.get(GetBy::Last).unwrap().title, "last");
}

#[test]
fn get_last_on_empty_returns_none() {
    let manager = Manager::default();
    assert!(manager.get(GetBy::Last).is_none());
}

// --- get_mut ---

#[test]
fn get_mut_allows_title_update() {
    let mut manager = manager_with_tasks(&["old"]);
    manager.get_mut(0usize).unwrap().title = "new".into();
    assert_eq!(manager.get(0usize).unwrap().title, "new");
}

#[test]
fn get_mut_on_missing_returns_none() {
    let mut manager = Manager::default();
    assert!(manager.get_mut(0usize).is_none());
}

// --- remove ---

#[test]
fn remove_by_index_decreases_count() {
    let mut manager = manager_with_tasks(&["a", "b"]);
    manager.remove(0usize).unwrap();
    assert_eq!(manager.get_all().len(), 1);
}

#[test]
fn remove_by_index_removes_correct_task() {
    let mut manager = manager_with_tasks(&["keep", "remove"]);
    manager.remove(1usize).unwrap();
    assert_eq!(manager.get_all()[0].title, "keep");
}

#[test]
fn remove_by_uuid_removes_correct_task() {
    let mut manager = Manager::default();
    let task = make_task("target");
    let id = *task.get_id();
    manager.add(task);
    manager.add(make_task("bystander"));

    manager.remove(id).unwrap();

    assert_eq!(manager.get_all().len(), 1);
    assert_eq!(manager.get_all()[0].title, "bystander");
}

#[test]
fn remove_last_removes_final_task() {
    let mut manager = manager_with_tasks(&["first", "second"]);
    manager.remove(GetBy::Last).unwrap();
    assert_eq!(manager.get_all().len(), 1);
    assert_eq!(manager.get_all()[0].title, "first");
}

#[test]
fn remove_last_on_empty_returns_err() {
    let mut manager = Manager::default();
    assert!(manager.remove(GetBy::Last).is_err());
}

#[test]
fn remove_out_of_bounds_returns_err() {
    let mut manager = manager_with_tasks(&["only"]);
    assert!(manager.remove(99usize).is_err());
}

#[test]
fn remove_unknown_uuid_returns_err() {
    let mut manager = manager_with_tasks(&["a"]);
    assert!(manager.remove(uuid::Uuid::new_v4()).is_err());
}

// --- clear ---

#[test]
fn clear_all_tasks_empties_manager() {
    let mut manager = manager_with_tasks(&["a", "b", "c"]);
    manager.clear_all_tasks();
    assert_eq!(manager.get_all().len(), 0);
}

#[test]
fn clear_on_empty_manager_is_safe() {
    let mut manager = Manager::default();
    manager.clear_all_tasks();
    assert_eq!(manager.get_all().len(), 0);
}

// --- get_all ---

#[test]
fn get_all_on_empty_returns_empty_slice() {
    let manager = Manager::default();
    assert!(manager.get_all().is_empty());
}

#[test]
fn get_all_returns_all_tasks() {
    let manager = manager_with_tasks(&["a", "b", "c"]);
    assert_eq!(manager.get_all().len(), 3);
}

// --- priority ---

#[test]
fn add_task_with_high_priority() {
    let mut manager = Manager::default();
    let mut task = Task::default();
    task.priority = Priority::High;
    manager.add(task);
    assert_eq!(manager.get(0usize).unwrap().priority, Priority::High);
}
