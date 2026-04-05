// tests/serialization_tests.rs
use rust_task_manager::tasks::taskstore::TaskStore;
use rust_task_manager::tasks::{Manager, Task};
use tempfile::NamedTempFile;

fn temp_path() -> (NamedTempFile, String) {
    let f = NamedTempFile::new().unwrap();
    let path = f.path().to_str().unwrap().to_owned();
    (f, path) // keep f alive so file isn't deleted
}

#[test]
fn save_and_load_round_trip() {
    let (_f, path) = temp_path();

    let mut manager = Manager::new(&path);
    let mut task = Task::default();
    task.title = "persisted".into();
    manager.add(task);
    manager.close().unwrap();

    let mut loaded = Manager::new(&path);
    loaded.open().unwrap();

    assert_eq!(loaded.get_all().len(), 1);
    assert_eq!(loaded.get_all()[0].title, "persisted");
}

#[test]
fn load_missing_file_is_ok() {
    let mut manager = Manager::new("nonexistent_file_xyz.json");
    let result = manager.open();
    assert!(result.is_ok());
    assert_eq!(manager.get_all().len(), 0);
}

#[test]
fn save_multiple_tasks_and_reload() {
    let (_f, path) = temp_path();
    let mut manager = Manager::new(&path);
    for title in &["a", "b", "c"] {
        let mut task = Task::default();
        task.title = title.to_string();
        manager.add(task);
    }
    manager.close().unwrap();

    let mut loaded = Manager::new(&path);
    loaded.open().unwrap();
    let titles: Vec<&str> = loaded.get_all().iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["a", "b", "c"]);
}

#[test]
fn save_preserves_uuid() {
    let (_f, path) = temp_path();
    let mut manager = Manager::new(&path);
    let task = Task::default();
    let original_id = *task.get_id();
    manager.add(task);
    manager.close().unwrap();

    let mut loaded = Manager::new(&path);
    loaded.open().unwrap();
    assert_eq!(loaded.get_all()[0].get_id(), &original_id);
}

#[test]
fn save_empty_manager_and_reload() {
    let (_f, path) = temp_path();
    let manager = Manager::new(&path);
    manager.close().unwrap();

    let mut loaded = Manager::new(&path);
    loaded.open().unwrap();
    assert_eq!(loaded.get_all().len(), 0);
}
