# Al-cat-raz — Project Definition

## 0. One-liner
A top-down 2D survival scavenger where a criminalized cat survives basements and
sewers under dog police rule, building power through crafting, stealth, social
play, and modular tinkering to eventually raid Al-cat-raz prison and City Hall.

## 1. High Concept
The city of [CITY_NAME] elects a new mayor who declares all cats criminals.
A dog police force hunts cats and imprisons them on the island prison
“Al-cat-raz”. Your tutor refuses to surrender you but can’t keep you safe,
forcing you into homelessness. You scavenge scraps, fight and evade police dogs,
compete or ally with other cats, and grow strong enough to:
1) destroy Al-cat-raz and free all cats
2) raid City Hall and hunt the mayor

## 2. Pillars (Non-negotiables)
1) Scavenge → Craft → Tinker → Survive
   - Scrap-driven economy: cans, glass, rope, stones, etc.
   - Multiple benches with different recipe sets and side-effects.
2) Noise-driven stealth
   - Loud actions produce noise that attracts dog police.
   - “Power has consequences”: more force often means more noise.
3) Systemic sandbox (social + combat)
   - Other cats are competitors or allies.
   - Multiplayer supports co-op and PvP as player choice.
4) Open endgame
   - Final objective is always attemptable; balance makes success late-game.

## 3. Art & Tone
- Style: low-res pixel art, cute cartoon silhouettes.
- Content: explicit gore effects (blood, dismemberment/decals as appropriate).
- Theme: oppression → survival → uprising.

## 4. Target Platforms
- First: Windows
- Later: Linux, macOS
- Multiplayer:
  - Singleplayer (server simulated in-process)
  - Listen server
  - Dedicated headless server

## 5. Player Count & PvP Policy
- Typical: 2–4 players
- Rare: up to ~10 (non-hard cap, but design target)
- PvP:
  - server config toggle `pvp_enabled`
  - combat rules enforce toggle server-side

## 6. Core Gameplay Systems (Committed)
### 6.1 Exploration
- Hand-crafted levels authored in LDtk.
- Basements/sewers/streets as zones with transitions.

### 6.2 Survival Loop
- Scavenge loot piles, containers, corpses.
- Maintain resources (exact survival meters TBD; start minimal).

### 6.3 Combat (Top-down)
- Melee-centric baseline.
- Gore VFX is presentation; damage resolution is systemic and server-authoritative.

### 6.4 Crafting (Real-time seconds)
- Benches:
  - recipe restrictions by tags
  - craft speed multipliers
  - noise multipliers
- Recipes:
  - consume ingredients → produce outputs after `time_s`
  - crafting can emit noise and draw dogs

### 6.5 Stealth via Sound
- Loudness-based noise events drive dog police behavior.
- Dog AI reacts to:
  - investigate last heard location
  - chase/attack if visual contact
  - (later) coordination/escalation

### 6.6 Tinkering via Safe Modular Gadgets
- No RNG failure states for tinkering.
- Items expose gadget slots; gadgets attach/detach at benches.
- Gadgets may:
  - modify stats (damage, cooldown, armor, etc.)
  - add traits (bleed, poison, knockback)
  - alter loudness (stealth tradeoffs)

## 7. Modding (100% Requirement)
Mods must support:
- data-driven content (items, recipes, benches, loot tables, factions, enemies)
- scripting for behaviors (server-side only)
- map additions (LDtk levels referencing mod IDs)

Key rule:
- Gameplay scripts execute only on the authoritative server.
- Clients render replicated outcomes, preventing cheating/desync.

## 8. Success Criteria (Definition of Done — Product Level)
- Playable start → mid → endgame with Al-cat-raz raid and City Hall raid.
- Endgame is always attemptable but realistically winnable late-game.
- Dedicated server supports 2–4 players reliably (rare ~10 acceptable).
- PvP toggle works and is enforced server-side.
- Mods can add data + scripts; server enforces mod hash matching.

## 9. Out of Scope (Initial Release)
- Rollback netcode.
- Complex deterministic lockstep simulation.
- Full in-game level editor (LDtk is used as an external data authoring tool).
- Web build target (revisit later).

## 10. Roadmap Overview (Milestones)
M0: Bootstrap repo + CI + dev ergonomics
M1: Vertical slice — movement, loot, noise, dog AI, death loop
M2: Crafting v1 — benches + recipes + noise integration
M3: Tinkering v1 — gadget slots + attach/detach + modifiers
M4: Modding v1 — RON data + Rhai scripts + sandbox + hashing
M5: Multiplayer v1 — dedicated server + replication + PvP toggle
M6: Expansion — progression, factions/social, content breadth
M7: Endgame — prison raid + City Hall + balance pass
M8: Polish/Release — performance, UX, saves, mod docs, packaging

## 11. Major Risks & Mitigations
- Bevy/plugin churn:
  - pin versions, upgrade only at milestone boundaries.
- Scripting performance/security:
  - server-only execution, strict sandbox budgets, tiny API surface.
- Multiplayer scope creep:
  - authoritative snapshots + interpolation first; prediction later if needed.
- Balance complexity:
  - in-game debug UI, data-driven tuning, telemetry logs.