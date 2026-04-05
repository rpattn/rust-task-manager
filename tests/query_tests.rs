// tests/query_tests.rs
use rust_task_manager::tasks::taskstore::{QueryOptions, SortOrder, TaskField, TaskStore};
use rust_task_manager::tasks::task::Priority;
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
    page: usize,
    page_size: usize,
    sort_field: TaskField,
    sort_order: SortOrder,
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
        page: 0,
        page_size: 100,
        sort_field: TaskField::Created,
        sort_order: SortOrder::Asc,
        filter: Some(field),
        value: Some(val.into()),
    }
}

// --- sort ---

#[test]
fn sort_by_title_asc() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(0, 10, TaskField::Title, SortOrder::Asc)));
    let titles: Vec<&str> = tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"]);
}

#[test]
fn sort_by_title_desc() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(0, 10, TaskField::Title, SortOrder::Desc)));
    let titles: Vec<&str> = tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["Elderberry", "Date", "Cherry", "Banana", "Apple"]);
}

#[test]
fn sort_by_priority_asc() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(0, 10, TaskField::Priority, SortOrder::Asc)));
    let priorities: Vec<&Priority> = tasks.iter().map(|t| &t.priority).collect();
    assert_eq!(priorities[0], &Priority::Low);
    assert_eq!(priorities[4], &Priority::High);
}

#[test]
fn sort_by_priority_desc() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(0, 10, TaskField::Priority, SortOrder::Desc)));
    assert_eq!(tasks[0].priority, Priority::High);
    assert_eq!(tasks[4].priority, Priority::Low);
}

#[test]
fn sort_by_created_asc_preserves_insertion_order() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(0, 10, TaskField::Created, SortOrder::Asc)));
    let titles: Vec<&str> = tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["Banana", "Apple", "Cherry", "Date", "Elderberry"]);
}

// --- pagination ---

#[test]
fn pagination_first_page() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(0, 2, TaskField::Title, SortOrder::Asc)));
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].title, "Apple");
    assert_eq!(tasks[1].title, "Banana");
}

#[test]
fn pagination_second_page() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(1, 2, TaskField::Title, SortOrder::Asc)));
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].title, "Cherry");
    assert_eq!(tasks[1].title, "Date");
}

#[test]
fn pagination_last_page_partial() {
    let store = store_with_tasks();
    // 5 tasks, page size 2, page 2 = last item only
    let tasks = store.get_all(Some(&query(2, 2, TaskField::Title, SortOrder::Asc)));
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "Elderberry");
}

#[test]
fn pagination_beyond_end_returns_empty() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(99, 10, TaskField::Title, SortOrder::Asc)));
    assert!(tasks.is_empty());
}

#[test]
fn pagination_page_size_larger_than_total() {
    let store = store_with_tasks();
    let tasks = store.get_all(Some(&query(0, 100, TaskField::Title, SortOrder::Asc)));
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
    assert_eq!(tasks.len(), 5); // all default to Todo
}

#[test]
fn filter_by_status_complete() {
    let mut store = store_with_tasks();
    use rust_task_manager::tasks::task::{Status, TaskEdit};
    store.edit(0usize, TaskEdit { title: None, priority: None, status: Some(Status::Complete) }).unwrap();
    let tasks = store.get_all(Some(&query_with_filter(TaskField::Status, "done")));
    assert_eq!(tasks.len(), 1);
}

// --- sort + filter combined ---

#[test]
fn filter_then_sort() {
    let store = store_with_tasks();
    let q = QueryOptions {
        page: 0,
        page_size: 10,
        sort_field: TaskField::Title,
        sort_order: SortOrder::Desc,
        filter: Some(TaskField::Priority),
        value: Some("low".into()),
    };
    let tasks = store.get_all(Some(&q));
    assert_eq!(tasks.len(), 2);
    // sorted desc: Date before Banana
    assert_eq!(tasks[0].title, "Date");
    assert_eq!(tasks[1].title, "Banana");
}

// --- sort + filter + pagination combined ---

#[test]
fn filter_sort_paginate() {
    let store = store_with_tasks();
    let q = QueryOptions {
        page: 0,
        page_size: 1,
        sort_field: TaskField::Title,
        sort_order: SortOrder::Asc,
        filter: Some(TaskField::Priority),
        value: Some("high".into()),
    };
    let tasks = store.get_all(Some(&q));
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "Apple"); // first high priority alphabetically
}
