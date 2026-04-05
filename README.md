# rust-task-manager

A terminal-based task manager written in Rust. Tasks are persisted locally as JSON, with a pluggable store architecture designed to grow into a full backend service.

## Features

- Add, edit, remove, and complete tasks
- Set task priority (Low, Medium, High) and status (Todo, Complete)
- Look up tasks by UUID or list index
- Filter by status or priority, sort by any field, paginate results
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

Tasks are saved to `out/tasks.json` by default. The file is created on first write.

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
cargo run -- list --filter priority --value high
cargo run -- list --sort priority --order desc
cargo run -- list --page 1 --size 10
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
| JSON store | `src/tasks/jsonstore.rs` | File-backed implementation |
| Basic store | `src/tasks/basicstore.rs` | In-memory implementation (used in tests) |
| Display | `src/display` | Table rendering via `comfy_table` |

The `TaskStore` trait is the core abstraction - all command logic is written against the trait, not any concrete backend. This is what allows the store to be swapped out (JSON  SQLite) without touching business logic.

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

The planned trajectory is CLI  API server  richer data model, each phase building on the last.

### Done

- [x] `clap` for argument parsing
- [x] Add, list, complete, remove, get, clear commands
- [x] `edit` command - update title, priority, or status by id
- [x] UUID + index based lookup throughout
- [x] `--priority` flag on `add`
- [x] `--last` flag on `remove`
- [x] `TaskStore` trait for pluggable backends
- [x] `JsonStore` with dirty-flag optimised writes
- [x] `BasicStore` in-memory implementation (tests)
- [x] Structured error types with `TaskStoreError` / `JsonStoreError`
- [x] Comprehensive test suite
- [x] Filter `list` by status or priority (`--filter status --value todo`)
- [x] Sort on list (`--sort priority --order desc`)
- [x] Pagination (`--page`, `--size`)

### Phase 1 - Config

- [ ] Config file (`~/.config/rust-task-manager/config.toml`) via `dirs` + `toml` crates
- [ ] Configurable storage path (replaces hardcoded `out/tasks.json`)
- [ ] Configurable defaults (page size, sort field, sort order)
- [ ] Display preferences (colour on/off)
- [ ] Config-driven backend selection (`backend = "json"` / `"sqlite"`)

### Phase 2 - SQLite Backend

- [ ] Fix `TaskStore` trait object safety (`impl IntoGetBy`  `GetBy` in method signatures)
- [ ] `SqliteStore` implementing `TaskStore` via `sqlx`
- [ ] Schema migrations
- [ ] Runtime backend selection from config (`Box<dyn TaskStore>`)

### Phase 3 - Generic Command Output

- [ ] Replace `CommandResult` with `CommandOutput` enum (`TaskList`, `SingleTask`, `Confirmation`, `Empty`)
- [ ] Decouple business logic from presentation entirely
- [ ] CLI handler serialises `CommandOutput` to table
- [ ] Foundation for API handlers consuming the same logic

### Phase 4 - REST API

- [ ] `axum` HTTP server
- [ ] REST handlers consuming `CommandOutput`
- [ ] JSON responses
- [ ] Use SQLite backend (JSON store unsuitable for concurrent requests)

### Phase 5 - Authentication

- [ ] JWT validation middleware via `jsonwebtoken` crate
- [ ] Zitadel as identity provider (JWKS-based token verification)
- [ ] `AuthUser` axum extractor - handlers declare it as a parameter
- [ ] Scope tasks to owner via `sub` claim from JWT (no separate users table needed)
- [ ] Role-based access if needed (claims-driven)

### Phase 6 - GraphQL API

- [ ] `async-graphql` + `async-graphql-axum` integration
- [ ] GraphQL schema over existing task types
- [ ] Query depth limiting (built into `async-graphql`)
- [ ] Inherits auth middleware from Phase 5
- [ ] Introspection disabled in production

### Phase 7 - Richer Task Model

This is where GraphQL starts earning its keep - nested and relational data is where it pulls ahead of REST.

- [ ] Subtasks (`parent_id`  recursive `Task` type in GraphQL)
- [ ] Tags (many-to-many)
- [ ] Due dates + `overdue` query
- [ ] Comments (append-only log per task)
- [ ] `assigned_to` user (pulled from JWT claims, not a separate users table)
- [ ] Split `Task` into db model (`TaskRow`) and domain model (`Task`) as nesting grows

### Phase 8 - TUI (optional)

- [ ] `ratatui` event loop
- [ ] Scrollable task list with keyboard navigation
- [ ] Inline task creation and editing
- [ ] Priority and status indicators
