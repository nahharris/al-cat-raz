# Al-cat-raz Project Roadmap

This document outlines the development milestones for **Al-cat-raz**, a top-down 2D survival scavenger built with Bevy.

## Core Pillars
1. **Scavenge → Craft → Tinker → Survive**: Scrap-driven economy with modular item tinkering.
2. **Noise-driven Stealth**: Systemic AI that reacts to loudness and sound events.
3. **Systemic Sandbox**: Multiplayer co-op and PvP where cats survive under dog police rule.
4. **Open Endgame**: Final objectives (Al-cat-raz raid, City Hall) are always attemptable.

---

## Development Milestones

### M0: Bootstrap & Foundations (Current Focus)
**Goal**: Establish repository structure, CI/CD, and core developer ergonomics.
- [x] Multi-crate workspace setup (`core`, `client`, `server`, `modkit`).
- [x] CI pipeline for Windows (fmt, clippy, tests).
- [x] Bevy version pinning policy (0.17.x) and dependency matrix.
- [x] Basic "Hello World" networking (Replicon + Renet).

### M1: Vertical Slice — Survival Loop
**Goal**: Implement the core gameplay loop in a singleplayer context.
- [ ] Character movement and top-down camera.
- [ ] LDtk level loading integration (`bevy_ecs_ldtk`).
- [ ] Loot system (scavenging piles, containers).
- [ ] Noise system (actions produce sound events).
- [ ] Basic Dog AI (investigate noise, chase visual contact).

### M2: Crafting v1 — The Bench System
**Goal**: Data-driven crafting with systemic consequences.
- [ ] Workbenches with tag-based recipe restrictions.
- [ ] Real-time crafting timers.
- [ ] Noise integration (crafting makes sound based on bench/recipe).
- [ ] Basic inventory management.

### M3: Tinkering v1 — Modular Gadgets
**Goal**: Deep item customization without RNG failure states.
- [ ] Gadget slot system for items.
- [ ] Attach/Detach mechanics at workbenches.
- [ ] Stat modifiers (damage, cooldown, armor).
- [ ] Trait modifiers (bleed, poison, loudness tradeoffs).

### M4: Modding v1 — Data & Scripting
**Goal**: 100% moddable content baseline.
- [ ] RON-based data schemas for items, benches, and recipes.
- [ ] Rhai scripting host (server-side only).
- [ ] Mod manifest parsing and loading order.
- [ ] Content hashing for multiplayer synchronization.

### M5: Multiplayer v1 — Authoritative Networking
**Goal**: Stable dedicated server support and PvP toggle.
- [ ] Server-authoritative simulation (combat, crafting, AI).
- [ ] Client interpolation and intent-based commands.
- [ ] `pvp_enabled` server toggle and enforcement.
- [ ] Mod hash matching/verification on join.

### M6: Expansion — Factions & Social
**Goal**: Adding depth to the systemic sandbox.
- [ ] Cat NPCs (allies and competitors).
- [ ] Faction/Reputation system.
- [ ] Dialogue and quest data schemas.
- [ ] Content breadth pass (more items, gadgets, and dog types).

### M7: Endgame — The Uprising
**Goal**: Implement the final objectives and balance the difficulty curve.
- [ ] Al-cat-raz prison raid map and mechanics.
- [ ] City Hall / Mayor hunt endgame loop.
- [ ] Global balance pass (tuning loot tables and AI difficulty).

### M8: Polish & Release
**Goal**: Production-ready stability and UX.
- [ ] Performance optimization (FixedUpdate tuning).
- [ ] Save/Load system with migration support.
- [ ] Final UI/UX pass (menus, HUD, inventory).
- [ ] Modding documentation and packaging tools.

---

## Technical Constraints
- **Platform**: Windows first, Linux/macOS later.
- **Networking**: Server-authoritative (Snapshots + Interpolation).
- **Scripting**: Rhai (Server-only sandbox, no IO).
- **Versioning**: Pinned Bevy 0.17.3 baseline to maintain plugin compatibility.

## Success Criteria (Definition of Done)
- [ ] Playable from start to endgame (Prison/City Hall raids).
- [ ] Dedicated server supports 2–4 players reliably (~10 rare).
- [ ] PvP toggle is strictly enforced server-side.
- [ ] 100% of game data and behaviors are moddable.
