use rust_task_manager::tasks::task::TaskEdit;
// tests/serialization_tests.rs
use rust_task_manager::tasks::taskstore::TaskStore;
use rust_task_manager::tasks::{JsonStore, Task};
use tempfile::NamedTempFile;

fn temp_path() -> (NamedTempFile, String) {
    let f = NamedTempFile::new().unwrap();
    let path = f.path().to_str().unwrap().to_owned();
    (f, path) // keep f alive so file isn't deleted
}

#[test]
fn save_and_load_round_trip() {
    let (_f, path) = temp_path();

    let mut manager = JsonStore::new(&path);
    let mut task = Task::default();
    task.title = "persisted".into();
    manager.add(task);
    manager.close().unwrap();

    let mut loaded = JsonStore::new(&path);
    loaded.open().unwrap();

    assert_eq!(loaded.get_all(None).len(), 1);
    assert_eq!(loaded.get_all(None)[0].title, "persisted");
}

#[test]
fn load_missing_file_is_ok() {
    let mut manager = JsonStore::new("nonexistent_file_xyz.json");
    let result = manager.open();
    assert!(result.is_ok());
    assert_eq!(manager.get_all(None).len(), 0);
}

#[test]
fn save_multiple_tasks_and_reload() {
    let (_f, path) = temp_path();
    let mut manager = JsonStore::new(&path);
    for title in &["a", "b", "c"] {
        let mut task = Task::default();
        task.title = title.to_string();
        manager.add(task);
    }
    manager.close().unwrap();

    let mut loaded = JsonStore::new(&path);
    loaded.open().unwrap();
    let titles: Vec<String> = loaded.get_all(None).into_iter().map(|t| t.title).collect();
    assert_eq!(titles, vec!["a", "b", "c"]);
}

#[test]
fn save_preserves_uuid() {
    let (_f, path) = temp_path();
    let mut manager = JsonStore::new(&path);
    let task = Task::default();
    let original_id = *task.get_id();
    manager.add(task);
    manager.close().unwrap();

    let mut loaded = JsonStore::new(&path);
    loaded.open().unwrap();
    assert_eq!(loaded.get_all(None)[0].get_id(), &original_id);
}

#[test]
fn save_empty_manager_and_reload() {
    let (_f, path) = temp_path();
    let mut manager = JsonStore::new(&path);
    manager.close().unwrap();

    let mut loaded = JsonStore::new(&path);
    loaded.open().unwrap();
    assert_eq!(loaded.get_all(None).len(), 0);
}

#[test]
fn edit_persists_after_reload() {
    let (_f, path) = temp_path();
    let mut manager = JsonStore::new(&path);
    manager.add(Task::default());
    manager
        .edit(
            0usize,
            TaskEdit {
                title: Some("edited".into()),
                priority: None,
                status: None,
            },
        )
        .unwrap();
    manager.close().unwrap();

    let mut loaded = JsonStore::new(&path);
    loaded.open().unwrap();
    assert_eq!(loaded.get_all(None)[0].title, "edited");
}

#[test]
fn edit_persists_without_prior_mutation() {
    let (_f, path) = temp_path();

    let mut manager = JsonStore::new(&path);
    manager.add(Task::default());
    manager.close().unwrap();

    let mut loaded = JsonStore::new(&path);
    loaded.open().unwrap();
    loaded
        .edit(
            0usize,
            TaskEdit {
                title: Some("edited".into()),
                priority: None,
                status: None,
            },
        )
        .unwrap();
    loaded.close().unwrap();

    let mut reloaded = JsonStore::new(&path);
    reloaded.open().unwrap();
    assert_eq!(reloaded.get_all(None)[0].title, "edited");
}
