# CLIToDo

CLIToDo is a small command-line todo application written in Rust. It provides a lightweight way to manage tasks from the terminal and includes functionality to add, list, complete, and persist tasks.

Status
- Language: Rust
- Work-in-progress: core data structures and load/save functionality have been implemented (see commit history).

Features (expected / implemented)
- Create tasks with a short description.
- List tasks (pending, completed, or all).
- Mark tasks as completed.
- Remove tasks.
- Persist tasks to disk and load them on startup (save/load functionality implemented).

Requirements
- Rust toolchain (recommended: latest stable)
  - Install from https://rustup.rs
- Cargo (comes with rustup)

Installation and build
1. Clone the repository:
   git clone https://github.com/nik200503/CLIToDo.git
   cd CLIToDo

2. Build in debug mode:
   cargo build

3. Build release binary:
   cargo build --release
   # Release binary will be in target/release/

Running (development)
- Run via cargo:
  cargo run -- <command> [args]

Example workflow (common commands)
Note: Exact command names and flags may differ slightly — check the binary help (`--help`) or open the source to confirm details.

- Add a new task:
  cargo run -- add "Buy groceries"

- List tasks:
  cargo run -- list
  cargo run -- list --all
  cargo run -- list --completed

- Mark a task complete:
  cargo run -- done <task-id>

- Remove a task:
  cargo run -- remove <task-id>

- Save tasks to disk (if not automatic):
  cargo run -- save

- Load tasks from disk:
  cargo run -- load

- Show help:
  cargo run -- --help

Persistence
- The project contains load/save functionality (implemented). The exact file used for persistence (e.g., `tasks.json`, an XDG data directory, or a different path) is set in the code — inspect the source (look for load/save or file path constants) to see the exact location and format (likely JSON or similar).

Development workflow
- Format:
  cargo fmt

- Lint:
  cargo clippy

- Run tests:
  cargo test

Contributing
- See CONTRIBUTING.md for guidelines. In short:
  - Open an issue for significant changes or features.
  - Create a feature branch, commit clearly, and open a pull request.
  - Keep changes small and focused.
  - Add tests where appropriate.

Project structure (typical for Rust)
- Cargo.toml — project manifest
- src/ — Rust source files (main.rs and modules)
- src/commands, src/task, or similar — where CLI parsing and task data structures live

Roadmap / ideas
- Improve CLI argument parsing and add helpful flags (priority, due date).
- Add syncing or export/import (CSV).
- Add unit and integration tests for persistence and CLI behavior.
- Add more robust error handling and better UX for interactive flows.

License
- No license file found in the repository. Add a LICENSE file (MIT, Apache-2.0, or other) to clarify how the project may be used.

Contact
- Repository: https://github.com/nik200503/CLIToDo
- Author (commit history): nik200503
