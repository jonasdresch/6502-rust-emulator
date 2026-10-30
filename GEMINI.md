# Agent Rules for 6502-rust-emulator

## Git Branching

- Always branch from `main` unless explicitly told otherwise.
- Use the following naming conventions:
  - `feat/<short-description>` for new features or enhancements.
  - `fix/<short-description>` for bug fixes.
  - `chore/<short-description>` for maintenance tasks (deps, config, CI, etc.).
- Never commit directly to `main`.
- Always push the branch and open a PR for review instead of merging locally.

## Testing

- Run `cargo test` after any change to `src/` or `tests/`.
- Skip running tests when only editing documentation files (`README.md`, comments, `GEMINI.md`, `.md` files in general).
- Always run tests before creating a PR or declaring a task done.
- If tests fail, fix the failures before proceeding — do not open a PR with broken tests.

## Code Quality

- Run `cargo clippy` after changes to `src/` and resolve all warnings before committing.
- Run `cargo fmt` to format code before committing.
