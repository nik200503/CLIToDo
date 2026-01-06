# Contributing to CLIToDo

Thanks for your interest in contributing! This document explains how to propose changes, run the project locally, and submit a pull request.

How to contribute
1. Fork the repository and create a branch for your change:
   git checkout -b feature/my-feature

2. Make your changes on that branch. Keep changes small and focused.

3. Make sure code is formatted:
   cargo fmt

4. Run the linter and tests:
   cargo clippy
   cargo test

5. Commit with a clear message and push your branch:
   git add .
   git commit -m "Add feature: brief description"
   git push origin feature/my-feature

6. Open a pull request against the `main` branch and describe:
   - What the change does.
   - Why it’s needed.
   - Any relevant notes about compatibility or migration.

Pull request guidance
- Include tests for new features or bug fixes when applicable.
- Explain design decisions in the PR description.
- Keep one logical change per PR when possible.

Reporting bugs and requesting features
- Open an issue with:
  - A clear title.
  - Steps to reproduce (for bugs).
  - Expected vs. actual behavior.
  - Platform and Rust toolchain version.

Code of conduct
- Be respectful and constructive. If you want, add a standard Code of Conduct file (e.g., Contributor Covenant).

Building and running locally
- Clone:
  git clone https://github.com/nik200503/CLIToDo.git
  cd CLIToDo

- Build:
  cargo build

- Run:
  cargo run -- --help

Thank you for helping improve CLIToDo!