# Al-cat-raz — Formal Project Definition (Record)

## 1) High Concept
**Al-cat-raz** is a **top-down 2D survival / scavenging** game with **low-res pixel art** that blends a **cute cartoon aesthetic** with **explicit gore**. You play a newly criminalized cat in a city where the mayor has deputized a dog police force to hunt cats and imprison them on the island prison **Al-cat-raz**. You survive by scavenging, crafting, stealthing, fighting, and social maneuvering—eventually becoming strong enough to destroy the prison, free the cats, and raid City Hall to hunt the mayor.

## 2) Vision & Pillars
### Core pillars
1) **Scavenge → Craft → Tinker → Survive**
   - Scraps become tools; tools become weapons; weapons become personalized via modular upgrades.
2) **Noise-driven stealth**
   - Loud actions (combat, crafting, breaking objects) create consequences: dogs investigate, patrol patterns shift, danger escalates.
3) **Systemic social sandbox**
   - Other cats compete for resources or form alliances. The player can cooperate or betray (especially in multiplayer).
4) **Open endgame**
   - The final objective is always available, but progression/balance makes success realistically “late game.”

### Tone & style
- **Visual:** pixel art, low-res, readable silhouettes, cartoon charm
- **Content:** deliberately gory combat/FX contrasted with cute characters
- **Theme:** oppression → survival → uprising

## 3) Target Platforms & Release Strategy
- **Primary (first):** Windows
- **Planned later:** Linux, macOS
- Multiplayer modes:
  - Singleplayer (local “server-in-process”)
  - Listen server
  - Dedicated headless server (same gameplay core)

## 4) Player Fantasy & Objectives
### Player role
A homeless cat, sheltered initially by a tutor who can’t keep you safe, forced into the city’s basements and sewers.

### Objectives
- **Short-term:** find food/scraps, avoid dog patrols, craft basic protection
- **Mid-term:** establish routes, alliances/rivalries, build specialized gear via tinkering
- **Late-game:** assault **Al-cat-raz**, free imprisoned cats, then raid City Hall and confront the mayor

## 5) Game Design Summary

### Genre & camera
- **Top-down 2D survival** with exploration, scavenging, crafting, combat, stealth, and social systems.

### World structure
- **Hand-crafted maps** authored in an external tilemap editor (LDtk), treated as data export (not a visual engine workflow).

### Progression
- Progression trees (skills/perks/unlocks) but **endgame always accessible**.

### Multiplayer rules
- **Co-op or PvP is player-driven**, controlled by a **server config toggle**:
  - `pvp_enabled = true/false`
- Server remains authoritative regardless.

---

## 6) Key Mechanics (Committed)

### 6.1 Crafting (real-time seconds)
- Multiple **crafting benches**, each with:
  - allowed recipe categories/tags
  - craft speed multipliers
  - noise multipliers
- Recipes combine scraps (glass shards, cans, ropes, stones, etc.) into:
  - weapons, armor, gadgets, consumables

**Important:** Crafting completion can emit **noise**, affecting stealth/AI.

### 6.2 Stealth via sound
- Actions emit `NoiseEvent(origin, loudness, kind, instigator)`
- Dog police AI uses hearing thresholds + investigation behaviors:
  - patrol → investigate noise → chase → attack → (optional) call backup
- Sound propagation implementation staged:
  - v1: radius + wall attenuation (fast)
  - v2: grid flood-fill propagation (better indoors/sewers)

### 6.3 Tinkering (safe modular gadgets)
- No “upgrade failure.” Instead:
  - Items can have **gadget slots**
  - Gadgets are **attachable/detachable** modules that add traits/modifiers
- Examples of emergent tradeoffs:
  - stronger gear may be louder (more dog attention)
  - bleeding effects increase lethality but increase risk

---

## 7) Modding (100% requirement) + Scripting

### Mod goals
- Mods can add:
  - items, benches, recipes, loot tables, enemies, factions, quests/dialogue
  - **scripts** that implement new behaviors (within a sandbox)

### Mod execution rule (critical)
- **All gameplay scripts run on the server only** (authoritative).
- Clients receive replicated results (status effects, damage, spawns), preventing desync and cheating.

### Mod packaging
Folder or zip:
```
mods/<mod_id>/
  mod.toml
  data/*.ron
  scripts/**/*.rhai
  assets/**/*
```

### Data format
- **RON + serde** for definitions (items, recipes, gadget slots, etc.)
- Versioned schema with:
  - `mod_api_version`
  - migrations as needed

### Scripting
- **Rhai** scripts attached via hook points (examples):
  - `on_craft_complete(ctx)`
  - `on_attach_gadget(ctx)`
  - `on_hit(ctx)`
  - `on_tick(ctx)` (restricted; budgeted)

### Sandbox requirements
- No file/network access.
- Script time/ops budget per tick/event.
- Deterministic RNG access exposed by engine (seeded), when needed.

### Multiplayer + mods compatibility
- Server advertises:
  - protocol version
  - ordered mod list with `{mod_id, version, content_hash}`
- Clients must match to join (baseline approach).

---

## 8) Technical Stack (Code-first, no visual engine editor)

### Language
- **Rust** (primary)

### Engine & core libraries (recommended)
- **Bevy** (ECS-first, code-driven)
- **Networking:** `bevy_replicon` (server-authoritative replication)
- **Transport:** Renet backend (via Replicon integration when version-compatible)
- **Maps:** **LDtk** + `bevy_ecs_ldtk`
- **Data:** `serde` + `ron`
- **Scripting:** `rhai`
- **Audio:** `bevy_kira_audio`
- **Dev UI/tools:** `bevy_egui`, `bevy_inspector_egui`

### Version pinning policy (important for stability)
Because Bevy + plugin compatibility shifts quickly:
- **Production policy:** pin exact versions in `Cargo.lock`, upgrade intentionally (milestone-based).
- **Baseline recommendation for early development:**
  - Choose the Bevy version that matches both:
    - `bevy_ecs_ldtk` you want
    - your Replicon+Renet backend integration
- **Upgrade plan:** once LDtk + Replicon transport integrations catch up, migrate to newer Bevy (e.g., Bevy 0.18 exists as of Jan 2026, but LDtk integration may lag).

(If you want, I can propose a single pinned set once you confirm whether you prefer “latest Bevy” vs “minimum churn.”)

---

## 9) Architecture & Repository Layout

### Core principle
**Pure gameplay logic is host-agnostic** (runs on dedicated server, listen server, or singleplayer-in-process).

### Crates
- `crates/game_core/`
  - ECS components + systems: survival, combat resolution, crafting, tinkering, AI, noise
  - No rendering/audio
- `crates/game_client/`
  - Rendering, input, UI, audio, interpolation, gore VFX
- `crates/game_server/`
  - Headless server, networking, persistence, mod loading
- `crates/modkit/`
  - RON schemas, validators, script host, mod manifest parsing

### Timing model
- Gameplay systems run in **FixedUpdate** (fixed dt, e.g., 30–60 Hz)
- Rendering/FX/UI in `Update`

---

## 10) Networking Model (Authoritative, co-op/PvP toggle)

### Authority
- Server simulates all game rules.
- Clients send **intent/commands**, not state (examples):
  - move intent
  - interact/craft request
  - attack request

### Replication
- Replicate minimal necessary components:
  - transforms/animation state
  - health/status effects
  - entities relevant to the client (interest management later)
- Clients interpolate remote entities; optional local movement prediction later.

### PvP toggle
- `pvp_enabled` config checked in server combat resolution for player→player damage.

---

## 11) Content Pipeline

### Maps
- Hand-authored in **LDtk**
- LDtk “entity markers” spawn gameplay entities:
  - benches, loot piles, sewer entrances, dog stations, exits, NPC cats, etc.
- Keep maps “dumb data”; game rules stay in code/data definitions.

### Assets
- Sprites + audio are standard Bevy assets.
- Mods can ship additional assets under `mods/<id>/assets/`.

---

## 12) Development Roadmap (Complete)

### Phase 0 — Project bootstrap (1–3 days)
**Deliverables**
- Monorepo with crates (`core/client/server/modkit`)
- CI (build + fmt + clippy)
- Basic logging + debug toggles
**Exit criteria**
- Builds on Windows; server and client binaries compile and run.

### Phase 1 — Vertical slice: survival + threat loop (2–4 weeks)
**Scope**
- One small LDtk map (basement + sewer transition)
- Player movement, collisions, basic pickup/drop
- Slot inventory
- Noise events + dog AI (patrol→investigate→chase→attack)
- Simple combat
**Exit criteria**
- Player can scavenge, make noise, be hunted, die, repeat.

### Phase 2 — Crafting v1 (1–2 weeks)
**Scope**
- Bench entities from LDtk
- Real-time crafting jobs (server-timed)
- Recipe system (RON)
- Craft completion emits noise
**Exit criteria**
- Player crafts a weapon/armor from scraps; dogs react to crafting noise.

### Phase 3 — Tinkering (gadgets) v1 (1–2 weeks)
**Scope**
- Gadget slots on items
- Attach/detach gadgets at tinkering bench
- Stat modifiers applied cleanly
**Exit criteria**
- Player can customize gear modularly with no failure states.

### Phase 4 — Modding foundation (2–4 weeks)
**Scope**
- Mod loader (manifest + RON definitions)
- Rhai scripting host + hook dispatch (server-only)
- Script sandbox budgets
**Exit criteria**
- A sample mod adds a gadget with an `on_hit` bleed script and works in-game.

### Phase 5 — Multiplayer foundation (2–6 weeks)
**Scope**
- Dedicated server runtime
- Connect 2–4 clients
- Replicate core gameplay state
- Command pipeline (move/interact/craft/attack)
- PvP config toggle
**Exit criteria**
- Two players can co-op scavenge + craft; toggling PvP works.

### Phase 6 — System expansion (ongoing, milestone-driven)
**Features**
- Progression trees + unlocks
- More enemy types + dog special units
- Social: alliances, rival cats, reputation/faction heat
- More benches/recipes/gadgets, meaningful tradeoffs (power vs noise vs durability)
**Exit criteria**
- Mid-game loop emerges (routes, gear builds, escalating police presence).

### Phase 7 — Endgame content (4–8+ weeks)
**Scope**
- Al-cat-raz prison raid (multi-stage)
- City Hall raid + mayor confrontation
- Balance for “attempt any time, win late”
**Exit criteria**
- Full start-to-finish playthrough possible.

### Phase 8 — Polish & release prep (4–8 weeks)
**Scope**
- Performance passes (AI + net bandwidth)
- UX/UI polish, accessibility options
- Save system stabilization + migrations
- Mod API documentation + examples
**Exit criteria**
- Release candidate quality, stable mod loading, stable multiplayer for target player counts.

### Phase 9 — Post-launch
- Mod ecosystem support, curated modding docs
- Dedicated server packaging improvements
- Content drops (new zones, benches, dog units, gadgets)

---

## 13) Risks & Mitigations (Known)
1) **Bevy plugin version churn**
   - Mitigation: pin versions, milestone-based upgrades, keep gameplay core isolated.
2) **Networking scope creep**
   - Mitigation: authoritative server + snapshots first; add prediction only if needed.
3) **Scripting performance/security**
   - Mitigation: server-only scripts, budgets, small API surface, no IO access.
4) **Balance complexity (open endgame)**
   - Mitigation: debug UI tooling, telemetry logs, late-game power curves tied to risk (noise/heat).

---

## 14) Definition of Done (high-level)
- Windows release runs smoothly; Linux/macOS builds validated
- Dedicated server supports typical 2–4 players (rare ~10) reliably
- PvP toggle works and is enforced server-side
- Mods can add data + scripted behaviors; server enforces mod hash matching
- Endgame is always accessible but realistically winnable only late-game
