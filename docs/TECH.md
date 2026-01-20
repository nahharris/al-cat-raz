# Al-cat-raz — Technical Design (Tech Stack + Architecture)

## 1) Primary Language
- Rust (core implementation language)

## 2) Engine & Libraries (Code-first)
- Engine: Bevy (ECS-first, code-driven)
- Map authoring: LDtk (external tool) + bevy_ecs_ldtk
- Networking: bevy_replicon (server-authoritative replication)
- Transport: bevy_replicon_renet (Renet backend)
- Data: serde + RON
- Scripting: Rhai (server-side only)
- Audio: bevy_kira_audio
- Dev UI: bevy_egui + bevy_inspector_egui

## 3) Version Matrix (Pinned Baseline)
Because LDtk + networking plugins must match Bevy versions, we maintain a
compatibility matrix.

### Baseline choice (stable / low-churn)

We’ll pin to Bevy 0.17.x and a plugin set known to align with it:

- bevy = 0.17.3 (pinned)
- bevy_ecs_ldtk = 0.13.0
- bevy_replicon = 0.37.* (matched by bevy_replicon_renet 0.13.0)
- bevy_replicon_renet = 0.13.0
- bevy_egui = 0.38.*
- bevy-inspector-egui = 0.35.0

Policy:
- pin exact versions in Cargo.lock
- only upgrade at milestone boundaries

## 4) Workspace Layout
Repository is a Cargo workspace with multiple crates:

- crates/core/
  - gameplay ECS components and systems
  - crafting, tinkering, AI, noise, combat resolution
  - scripting host + hook dispatch (engine-agnostic)
- crates/client/
  - rendering, input, UI, audio, gore VFX
  - interpolation for replicated entities
- crates/server/
  - headless server binary
  - networking setup, persistence, mod loading, script execution
- crates/modkit/
  - shared schemas (RON), validators, mod manifest parser, hashing

## 5) Simulation Model
- Gameplay runs on FixedUpdate (fixed timestep, e.g., 30–60 Hz).
- Rendering/UI in Update.
- Crafting uses real-time seconds via timers advanced by fixed dt.

## 6) Networking Architecture (Authoritative)
### Authority
- Server simulates all authoritative state:
  - movement results, combat resolution, crafting completion, loot spawns,
    reputation/faction changes, noise events, AI decisions
- Clients send intent/commands (not state):
  - MoveIntent, Interact, Attack, StartCraft, AttachGadget, DetachGadget

### Replication
- Replicate only what clients must render:
  - transforms, animation state
  - health/status effects
  - relevant entities (interest management later)
- Inventory replication:
  - full for owning player; summarized for others (design choice)

### PvP toggle
- Server config:
  - pvp_enabled: bool
- Combat system checks toggle for player→player damage.

## 7) Map Pipeline (LDtk)
- Levels authored in LDtk (hand-crafted).
- LDtk entity markers spawn gameplay ECS entities:
  - benches, loot piles, dog stations, exits/transitions, NPC cats, etc.
- Treat LDtk as data export; no reliance on a visual engine editor.

## 8) Mod System
### Loading
- Server loads mods from mods/ directory in a deterministic order:
  - base game content first
  - then mods in configured order
- Build a merged registry:
  - items, benches, recipes, loot tables, etc.
- Compute content hashes for multiplayer verification.

### Multiplayer compatibility
- Server broadcasts:
  - protocol version
  - ordered mod list {mod_id, version, content_hash}
- Client must match to join.

## 9) Scripting Model (Rhai)
- Scripts are executed ONLY on the server.
- Scripts attach to content definitions via file references.
- Sandbox:
  - no filesystem/network access
  - per-event instruction/time budgets
  - limited API surface exposed to scripts

## 10) Save/Load (planned)
- Versioned save format (SaveVersion field + migrations).
- Server-authoritative saves; singleplayer uses in-process server save.

## 11) Dev Ergonomics
- In-game tuning menus:
  - bevy_egui for debug panels
  - bevy_inspector_egui for inspecting ECS state
- CI:
  - build client/server on Windows
  - run fmt + clippy
  - (later) cross-compile Linux/macOS

## 12) Workspace Bootstrap Snippet
Root Cargo workspace sketch:

[workspace]
members = [
  "crates/core",
  "crates/client",
  "crates/server",
  "crates/modkit",
]
resolver = "2"