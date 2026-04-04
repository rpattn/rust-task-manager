// tests/serialization_tests.rs
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

    let mut manager = Manager::default();
    let mut task = Task::default();
    task.title = "persisted".into();
    manager.add(task);
    manager.save_tasks(&path).unwrap();

    let mut loaded = Manager::default();
    loaded.load_tasks(&path).unwrap();

    assert_eq!(loaded.get_all().len(), 1);
    assert_eq!(loaded.get_all()[0].title, "persisted");
}

#[test]
fn load_missing_file_is_ok() {
    let mut manager = Manager::default();
    let result = manager.load_tasks("nonexistent_file_xyz.json");
    assert!(result.is_ok());
    assert_eq!(manager.get_all().len(), 0);
}

#[test]
fn save_multiple_tasks_and_reload() {
    let (_f, path) = temp_path();

    let mut manager = Manager::default();
    for title in &["a", "b", "c"] {
        let mut task = Task::default();
        task.title = title.to_string();
        manager.add(task);
    }
    manager.save_tasks(&path).unwrap();

    let mut loaded = Manager::default();
    loaded.load_tasks(&path).unwrap();

    let titles: Vec<&str> = loaded.get_all().iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["a", "b", "c"]);
}

#[test]
fn save_preserves_uuid() {
    let (_f, path) = temp_path();

    let mut manager = Manager::default();
    let task = Task::default();
    let original_id = *task.get_id();
    manager.add(task);
    manager.save_tasks(&path).unwrap();

    let mut loaded = Manager::default();
    loaded.load_tasks(&path).unwrap();

    assert_eq!(loaded.get_all()[0].get_id(), &original_id);
}

#[test]
fn save_empty_manager_and_reload() {
    let (_f, path) = temp_path();

    let manager = Manager::default();
    manager.save_tasks(&path).unwrap();

    let mut loaded = Manager::default();
    loaded.load_tasks(&path).unwrap();
    assert_eq!(loaded.get_all().len(), 0);
}
