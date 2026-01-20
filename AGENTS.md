# Agent Guidelines for Al-cat-raz

## Scope
These instructions apply to the entire repository.

## Repo overview
- Rust workspace targeting Windows first (Linux/macOS later).
- Main crates:
  - `crates/core`: shared gameplay ECS types and systems (no rendering).
  - `crates/client`: Bevy client binary.
  - `crates/server`: headless server binary.
  - `crates/modkit`: mod schemas + hashing + manifest parsing.
- Mods live under `mods/` and are data-driven (RON/TOML + Rhai scripts).

## Build, lint, and test commands
- Workspace build: `cargo build --workspace`
- Build a single crate: `cargo build -p core`
- Run client: `cargo run -p client -- --server 127.0.0.1:5000`
- Run server: `cargo run -p server -- --addr 127.0.0.1:5000`
- Format: `cargo fmt`
- Lint (clippy): `cargo clippy --all-targets --all-features`
- Tests (workspace): `cargo test --workspace`
- Tests (single crate): `cargo test -p core`
- Single test by name (crate): `cargo test -p core test_name`
- Single test by module path: `cargo test -p core module::tests::test_name`
- Single integration test: `cargo test -p core --test integration_name`
- Doc tests (if added): `cargo test -p core --doc`

## Code style and conventions
### Formatting
- Use `cargo fmt` (rustfmt defaults). Do not hand-align whitespace.
- Keep diffs minimal and avoid unrelated reformatting.

### Imports
- Order groups as: `std` → external crates → local crates/modules.
- Prefer `use foo::{bar, baz};` for grouped imports.
- Use `crate::` or `super::` for local paths; avoid absolute `::` unless needed.

### Naming
- Types/traits/enums: `UpperCamelCase`.
- Functions/variables/modules/fields: `snake_case`.
- Constants: `SCREAMING_SNAKE_CASE`.
- Mod IDs and content IDs: `snake_case` strings (per `docs/MODDING.md`).

### Types and APIs
- Use explicit types for public structs and schema fields.
- Prefer `Option<T>` for optional fields; use `#[serde(default)]` for empty vecs.
- Favor `PathBuf` for filesystem paths in schemas and manifests.
- Keep serialization formats stable (RON/TOML schema is part of mod API).

### Error handling
- Binaries use `anyhow::Result` for top-level errors.
- Libraries should use typed errors (`thiserror`) where API clarity matters.
- Prefer `?` for propagation; reserve `expect` for unrecoverable invariants.
- Avoid `unwrap` outside tests unless there is a clear invariant comment.

### Logging and diagnostics
- Use `debug!`, `info!`, `warn!`, `error!` from Bevy logging macros.
- Keep log lines short and actionable; avoid noisy per-frame logs.

### Bevy/ECS patterns
- Register systems via `add_systems(Startup, ...)` and `add_systems(Update, ...)`.
- Components should be `#[derive(Component)]` and small, data-only.
- Use resources for shared state; keep them `#[derive(Resource)]` when possible.
- Server is authoritative; clients should send intent, not state.

### Networking
- `PROTOCOL_ID` lives in `crates/core` and must stay in sync across client/server.
- Keep replication types minimal and explicit (see `NetTransform`).

### Modding rules
- Scripts (Rhai) run server-side only.
- No filesystem/network access is exposed to scripts.
- Content hashes are computed deterministically; avoid nondeterministic ordering.

### Files and module layout
- Keep schema definitions in `crates/modkit/src/schema/`.
- Avoid cross-crate circular dependencies.
- Prefer smaller files/modules with focused responsibility.

## Testing guidance
- Add unit tests near the code they cover (`mod tests { ... }`).
- Use integration tests in `crates/<name>/tests/` when cross-module setup is needed.
- Keep tests deterministic; avoid relying on wall clock time.
- If a test needs data files, place them under `crates/<name>/tests/data/`.

## Documentation
- Update docs when behavior or schema changes impact modders or users.
- Primary docs live in `docs/` (TECH/PROJECT/MODDING).

## Agent workflow tips
- Check for additional `AGENTS.md` in subdirectories before editing files there.
- Do not touch `target/` or generated artifacts.
- Avoid adding new dependencies unless required for the task.
- Prefer minimal, well-scoped commits and changes.
