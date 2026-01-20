# Al-cat-raz

Top-down 2D survival scavenger.
Cute low-res pixel art + very gore graphics.
Server-authoritative multiplayer + 100% moddable content + server-side scripting.

Core docs:
- [PROJECT.md](docs/PROJECT.md) — game definition and roadmap
- [TECH.md](docs/TECH.md) — tech stack, architecture, version matrix
- [MODDING.md](docs/MODDING.md) — mod format, scripting rules, multiplayer compatibility

## Repo layout

- crates/core/   Gameplay ECS systems and shared types (no rendering/audio)
- crates/client/ Client binary (rendering, input, UI)
- crates/server/ Dedicated server binary (headless)
- crates/modkit/      Mod schemas, manifest parsing, hashing utilities

## Prereqs

- Rust stable (recommended via rust-toolchain.toml)
- Windows first (Linux/macOS later)

## Quick start (network hello world)

### 1) Run server
```bash
cargo run -p server -- --addr 127.0.0.1:5000
```
### 2) Run client (in another terminal)
```bash
cargo run -p client -- --server 127.0.0.1:5000
```

You should see a window with a red square whose position is replicated from the server (minimal Replicon+Renet wiring).

## Development notes

### Bevy version policy

We pin to a stable baseline (Bevy 0.17.x) because LDtk + Replicon transport

plugins are version-sensitive. Upgrades happen only at milestone boundaries.

## Mods

Baseline folder:
- mods/base/ contains initial example manifests and data (RON).

Server will eventually:
- load mods, merge registries, compute content hashes
- enforce exact mod-hash matching for multiplayer joins

## Useful commands

### Format:
```bash
	cargo fmt
```
### Lint:
```bash
	cargo clippy --all-targets --all-features
``` 
### Run tests:
```bash
	cargo test --workspace
```

### License

TBD (recommend dual MIT/Apache-2.0 for code, separate license for assets).

