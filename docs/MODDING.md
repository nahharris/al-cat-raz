# Al-cat-raz — Modding & Scripting (v1)

## 1) Goals
Mods must be able to add:
- Items, benches, recipes, loot tables
- Enemies, factions, quests/dialogue (later milestones)
- Behaviors via scripts (Rhai)
- Levels via LDtk projects/levels

Non-goal:
- Client-authoritative scripted gameplay (scripts run on server only)

## 2) Mod Package Layout
mods/<mod_id>/
  mod.toml
  data/
    items.ron
    benches.ron
    recipes.ron
    loot_tables.ron
  scripts/
    gadgets/*.rhai
    recipes/*.rhai
    ai/*.rhai
  assets/
    sprites/...
    sfx/...
    ldtk/...

## 3) Manifest: mod.toml (Proposed)
Required:
- mod_id = "gorekit"
- name = "Gore Kit"
- version = "1.0.0"
- mod_api_version = 1

Optional:
- dependencies = [{ mod_id = "base", version = ">=1.0.0" }]
- description = "..."
- author = "..."

## 4) Data Format
- RON files parsed with serde.
- All IDs are stable strings (snake_case recommended).

### Item example (conceptual)
- base item defs: weapons, armor, scraps
- gadget defs: attachable modules
- items can declare gadget slots

## 5) Scripting (Rhai)
### Rule: server-side only
Scripts execute on the authoritative server. Clients render replicated outcomes.

### Hook points (v1)
- on_craft_complete(ctx)
- on_attach_gadget(ctx)
- on_detach_gadget(ctx)
- on_hit(ctx)

(ctx is a map of primitive values only.)

### Exposed API (v1)
Engine functions available to scripts (server only):
- emit_noise(x, y, loudness, kind)
- apply_status(entity_id, status_id, stacks, duration_s)
- heal(entity_id, amount)
- spawn_item(item_id, count, x, y)
- log(text) [dev only]

### Budgets & safety
- Scripts have an instruction/time budget per invocation.
- No IO access from scripts.
- Engine provides RNG if/when needed (seeded).

## 6) Multiplayer Compatibility
- Server requires exact mod match:
  - protocol_version
  - ordered mod list with content_hash
- Clients not matching are rejected.

## 7) Example: Gadget Script
scripts/gadgets/jagged_glass.rhai

fn on_hit(ctx) {
  apply_status(ctx["victim_id"], "bleed", 1, 4.0);
  emit_noise(ctx["pos_x"], ctx["pos_y"], ctx["loudness"] + 2.0, "wet_hit");
}