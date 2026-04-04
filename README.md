# rust-task-manager
A terminal-based task manager written in Rust. Tasks are persisted locally as JSON.

## Features
- Add, remove, and complete tasks
- Set task priority (Low, Medium, High)
- Look up tasks by UUID or list index
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

Tasks are saved to `out/tasks.json` automatically. The file is created on first write.

## Usage

```bash
cargo run -- add "Buy milk"
cargo run -- list
cargo run -- complete <id|index>
cargo run -- remove <id|index>
cargo run -- remove --last
cargo run -- get <id|index>
cargo run -- clear --force
```

## Dependencies
| Crate | Purpose |
|---|---|
| `serde` / `serde_json` | JSON serialisation |
| `uuid` | Unique task IDs |
| `chrono` | Timestamps |
| `clap` | CLI argument parsing |
| `thiserror` | Structured error types |

---

## Roadmap

### Bug Fixes & Cleanup
- [x] Return `Err` from `load_tasks` on parse failure instead of swallowing it
- [x] Swap `get_all()` return type from `&Vec<Task>` to `&[Task]`
- [x] Remove redundant `'static` lifetime on `TASKS_FILENAME`
- [x] Replace `unwrap()` in `store/mod.rs` on `create_dir_all`
- [x] Implement `Default` for `Manager`

### CLI
- [x] `clap` for argument parsing
- [x] `task add <title>`
- [x] `task list`
- [x] `task complete <id|index>`
- [x] `task remove <id|index>`
- [x] UUID + index based lookup throughout
- [x] `--priority` flag on `add`

### Next
- [x] `edit` command — update title or priority by id
- [ ] Filter `list` by status or priority (`--done`, `--priority high`)

### Stretch
- [ ] Due dates with `--due` flag and `overdue` command
- [ ] Sort on list (`--sort priority`, `--sort created`)
- [ ] Config file for storage path (`~/.config/taskmanager/config.toml`)
- [ ] Swap JSON storage for SQLite via `rusqlite`

### TUI (future)
- [ ] Add `ratatui` and wire up event loop
- [ ] Scrollable task list with keyboard navigation
- [ ] Inline task creation
- [ ] Priority and status indicators
