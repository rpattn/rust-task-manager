// tests/query_tests.rs
use rust_task_manager::tasks::task::Priority;
use rust_task_manager::tasks::taskstore::{QueryOptions, SortOrder, TaskField, TaskStore};
use rust_task_manager::tasks::{BasicStore, Task};

fn make_task(title: &str, priority: Priority) -> Task {
    let mut task = Task::default();
    task.title = title.into();
    task.priority = priority;
    task
}

fn store_with_tasks() -> BasicStore {
    let mut store = BasicStore::default();
    store.add(make_task("Banana", Priority::Low));
    store.add(make_task("Apple", Priority::High));
    store.add(make_task("Cherry", Priority::Medium));
    store.add(make_task("Date", Priority::Low));
    store.add(make_task("Elderberry", Priority::High));
    store
}

fn query(
    page: Option<usize>,
    page_size: Option<usize>,
    sort_field: Option<TaskField>,
    sort_order: Option<SortOrder>,
) -> QueryOptions {
    QueryOptions {
        page,
        page_size,
        sort_field,
        sort_order,
        filter: None,
        value: None,
    }
}

fn query_with_filter(field: TaskField, val: &str) -> QueryOptions {
    QueryOptions {
        page: None,       // use default
        page_size: None,  // use default
        sort_field: None, // use default
        sort_order: None, // use default
        filter: Some(field),
        value: Some(val.into()),
    }
}

// --- sort ---

#[test]
fn sort_by_title_asc() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        None,
        None,
        Some(TaskField::Title),
        Some(SortOrder::Asc),
    )));
    let titles: Vec<&str> = tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(
        titles,
        vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"]
    );
}

#[test]
fn sort_by_title_desc() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        None,
        None,
        Some(TaskField::Title),
        Some(SortOrder::Desc),
    )));
    let titles: Vec<&str> = tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(
        titles,
        vec!["Elderberry", "Date", "Cherry", "Banana", "Apple"]
    );
}

#[test]
fn sort_by_priority_asc() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        None,
        None,
        Some(TaskField::Priority),
        Some(SortOrder::Asc),
    )));
    let priorities: Vec<&Priority> = tasks.iter().map(|t| &t.priority).collect();
    assert_eq!(priorities.first(), Some(&&Priority::Low));
    assert_eq!(priorities.last(), Some(&&Priority::High));
}

#[test]
fn sort_by_priority_desc() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        None,
        None,
        Some(TaskField::Priority),
        Some(SortOrder::Desc),
    )));
    assert_eq!(tasks.first().unwrap().priority, Priority::High);
    assert_eq!(tasks.last().unwrap().priority, Priority::Low);
}

#[test]
fn sort_by_created_asc_preserves_insertion_order() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        None,
        None,
        Some(TaskField::Created),
        Some(SortOrder::Asc),
    )));
    let titles: Vec<&str> = tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(
        titles,
        vec!["Banana", "Apple", "Cherry", "Date", "Elderberry"]
    );
}

// --- pagination ---

#[test]
fn pagination_first_page() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        Some(0),
        Some(2),
        Some(TaskField::Title),
        Some(SortOrder::Asc),
    )));
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].title, "Apple");
    assert_eq!(tasks[1].title, "Banana");
}

#[test]
fn pagination_second_page() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        Some(1),
        Some(2),
        Some(TaskField::Title),
        Some(SortOrder::Asc),
    )));
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].title, "Cherry");
    assert_eq!(tasks[1].title, "Date");
}

#[test]
fn pagination_last_page_partial() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        Some(2),
        Some(2),
        Some(TaskField::Title),
        Some(SortOrder::Asc),
    )));
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "Elderberry");
}

#[test]
fn pagination_beyond_end_returns_empty() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        Some(99),
        Some(10),
        Some(TaskField::Title),
        Some(SortOrder::Asc),
    )));
    assert!(tasks.is_empty());
}

#[test]
fn pagination_page_size_larger_than_total() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(
        Some(0),
        Some(100),
        Some(TaskField::Title),
        Some(SortOrder::Asc),
    )));
    assert_eq!(tasks.len(), 5);
}

#[test]
fn no_query_returns_all_tasks() {
    let store = store_with_tasks();
    let tasks = store.get_all(None);
    assert_eq!(tasks.len(), 5);
}

// --- filter ---

#[test]
fn filter_by_title_exact() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query_with_filter(TaskField::Title, "Apple")));
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "Apple");
}

#[test]
fn filter_by_title_no_match() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query_with_filter(TaskField::Title, "Mango")));
    assert!(tasks.is_empty());
}

#[test]
fn filter_by_priority_low() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query_with_filter(TaskField::Priority, "low")));
    assert_eq!(tasks.len(), 2);
    assert!(tasks.iter().all(|t| t.priority == Priority::Low));
}

#[test]
fn filter_by_priority_high() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query_with_filter(TaskField::Priority, "high")));
    assert_eq!(tasks.len(), 2);
    assert!(tasks.iter().all(|t| t.priority == Priority::High));
}

#[test]
fn filter_by_priority_case_insensitive() {
    let store = store_with_tasks();
    let lower = store.get_all(Some(&query_with_filter(TaskField::Priority, "high")));
    let upper = store.get_all(Some(&query_with_filter(TaskField::Priority, "HIGH")));
    assert_eq!(lower.len(), upper.len());
}

#[test]
fn filter_by_status_todo() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query_with_filter(TaskField::Status, "todo")));
    assert_eq!(tasks.len(), 5);
}

#[test]
fn filter_by_status_complete() {
    let mut store = store_with_tasks();
    use rust_task_manager::tasks::task::{Status, TaskEdit};

    store
        .edit(
            0,
            TaskEdit {
                title: None,
                priority: None,
                status: Some(Status::Complete),
            },
        )
        .unwrap();

    let tasks = store.get_all(Some(&query_with_filter(TaskField::Status, "done")));
    assert_eq!(tasks.len(), 1);
}

// --- combined ---

#[test]
fn filter_then_sort() {
    let store = store_with_tasks();
    let q = QueryOptions {
        page: None,
        page_size: None,
        sort_field: Some(TaskField::Title),
        sort_order: Some(SortOrder::Desc),
        filter: Some(TaskField::Priority),
        value: Some("low".into()),
    };

    let tasks = store.get_all(Some(&q));
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].title, "Date");
    assert_eq!(tasks[1].title, "Banana");
}

#[test]
fn filter_sort_paginate() {
    let store = store_with_tasks();
    let q = QueryOptions {
        page: Some(0),
        page_size: Some(1),
        sort_field: Some(TaskField::Title),
        sort_order: Some(SortOrder::Asc),
        filter: Some(TaskField::Priority),
        value: Some("high".into()),
    };

    let tasks = store.get_all(Some(&q));
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "Apple");
}
