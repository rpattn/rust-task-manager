# rust-task-manager
A terminal-based task manager written in Rust. Tasks are persisted locally as JSON.

## Features
- Add, edit, remove, and complete tasks
- Set task priority (Low, Medium, High) and status (Todo, Complete)
- Look up tasks by UUID or list index
- Persistent JSON storage with dirty-flag optimisation
- Pluggable store architecture (`TaskStore` trait)
- UUID-based task identification

## Requirements
- Rust 1.88+

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
cargo run -- add "Buy milk" --priority high
cargo run -- edit <id|index> --title "New Title"
cargo run -- edit <id|index> --priority medium
cargo run -- edit <id|index> --status complete
cargo run -- complete <id|index>
cargo run -- remove <id|index>
cargo run -- remove --last
cargo run -- get <id|index>
cargo run -- clear --force
```
Running with no subcommand lists all tasks.

## Architecture
The codebase is split into clean layers:

| Layer | Location | Responsibility |
|---|---|---|
| CLI parsing | `src/parser` | Argument parsing via `clap` |
| Commands | `src/commands.rs` | Business logic, returns `CommandResult` |
| Store trait | `src/tasks/taskstore.rs` | `TaskStore` interface |
| Json store | `src/tasks/jsonstore.rs` | File-backed implementation |
| Basic store | `src/tasks/basicstore.rs` | In-memory implementation (used in tests) |
| Display | `src/display` | Table rendering via `comfy_table` |

## Dependencies
| Crate | Purpose |
|---|---|
| `serde` / `serde_json` | JSON serialisation |
| `uuid` | Unique task IDs |
| `chrono` | Timestamps |
| `clap` | CLI argument parsing |
| `thiserror` | Structured error types |
| `comfy_table` | Terminal table rendering |

## Roadmap

### Done
- [x] `clap` for argument parsing
- [x] Add, list, complete, remove, get, clear commands
- [x] `edit` command — update title, priority, or status by id
- [x] UUID + index based lookup throughout
- [x] `--priority` flag on `add`
- [x] `--last` flag on `remove`
- [x] `TaskStore` trait for pluggable backends
- [x] `JsonStore` with dirty-flag optimised writes
- [x] `BasicStore` in-memory implementation
- [x] Structured error types with `TaskStoreError` / `JsonStoreError`
- [x] Comprehensive test suite

### Next
- [x] Filter `list` by status or priority (`--status todo`, `--priority high`)
- [x] Sort on list (`--sort priority`, `--sort created`)
- [ ] Config file for storage path (`~/.config/rust-task-manager/config.toml`)
- [x] Pagination (`--page`, `--page-size`)

### Stretch
- [ ] Due dates with `--due` flag and `overdue` command
- [ ] SQLite backend via `rusqlite`

### TUI (future)
- [ ] Add `ratatui` and wire up event loop
- [ ] Scrollable task list with keyboard navigation
- [ ] Inline task creation
- [ ] Priority and status indicators
