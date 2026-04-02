# rust-task-manager

A terminal-based task manager written in Rust. Tasks are persisted locally as JSON.

## Features

- Add, remove, and update tasks
- Set task priority (Low, Medium, High)
- Mark tasks as complete
- Persistent JSON storage
- UUID-based task identification

## Requirements

- Rust 1.75+

## Getting Started

```bash
git clone <repo>
cd rust-task-manager
cargo run
```

Tasks are saved to `out/tasks.json` automatically.

## Dependencies

| Crate | Purpose |
|---|---|
| `serde` / `serde_json` | JSON serialisation |
| `uuid` | Unique task IDs |
| `chrono` | Timestamps |

---

## Roadmap

### Bug Fixes & Cleanup
- [ ] Replace `unwrap()` in `store/mod.rs` with `?` for proper error propagation
- [ ] Return `Err` from `load_tasks` on parse failure instead of swallowing it
- [ ] Swap `get_all()` return type from `&Vec<Task>` to `&[Task]`
- [ ] Remove redundant `'static` lifetime on `TASKS_FILENAME`
- [ ] Implement `Default` for `Manager`

### CLI (next)
- [ ] Add `clap` for argument parsing
- [ ] `task add <title> --priority <low|medium|high>`
- [ ] `task list`
- [ ] `task done <id>`
- [ ] `task remove <id>`
- [ ] Switch from index-based to ID-based task lookup throughout

### TUI
- [ ] Add `ratatui` and wire up event loop
- [ ] Scrollable task list view
- [ ] Keyboard navigation (j/k or arrow keys)
- [ ] Inline task creation
- [ ] Priority and status indicators
- [ ] Highlight overdue or high priority tasks

### Storage
- [ ] Support multiple task lists / projects
- [ ] Configurable storage path via env or config file

### Stretch
- [ ] Due dates
- [ ] Filter and sort (by priority, status, date)
- [ ] Tags
