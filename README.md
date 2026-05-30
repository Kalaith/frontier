# Frontier Kingdom

Frontier Kingdom is a dark expedition card RPG built in Rust with Macroquad. You manage a fragile settlement, recruit adventurers, build facilities, choose branching mission routes, and survive turn-based card combat while stress and injuries persist between expeditions.

## Current Features

- Party formation with soldiers, scouts, healers, and mystics.
- Gendered adventurer portraits and class-specific card pools.
- Candlelit command-table base UI with top-level tabs, facility cards, action bar, and contextual detail panels.
- Embark preparation screen with party risk, mission briefing, readable locked requirements, and rewards.
- Branching mission maps with combat, event, rest, and boss nodes.
- Expedition route panel with anchored party status and node legend.
- Region-aware enemy spawning from JSON data.
- Turn-based card combat with central enemy intent, battle report preview, energy, block, status effects, and hover tooltips.
- Stress, resolve checks, trauma, heart attacks, injuries, death, and a graveyard.
- Kingdom facilities for healing, stress relief, recruitment, card learning, and the citadel win condition.
- Threat scaling, economy rewards, quest log, random kingdom events, save/load, and deck viewer.

## Content

- 33 cards, including Knowledge-unlockable advanced cards.
- 10 enemies across 6 AI patterns.
- 3 regions with region art and mission unlock requirements.
- Data-driven cards, enemies, missions, regions, and prompt metadata under `assets/`.

## Controls

- `Tab`: cycle base tabs.
- `1-9`: select roster, building, mission, path, or combat card depending on screen.
- `M`: form a party from the selected adventurer.
- `D`: view the selected adventurer's deck.
- `H`: heal at the Infirmary when built.
- `T`: reduce stress at the Chapel/Tavern when built.
- `U`: learn an advanced card at the Foundry when built.
- `R`: recruit from the Guild Hall.
- `Enter`: confirm selection, construct, embark, choose event, or play selected card.
- `Space`: advance missions or confirm paths.
- `A/D` or `Left/Right`: choose between available mission paths.
- `E`: end combat turn.
- `Esc`: close overlays, cancel, retreat, or return.
- `F5` / `F9`: save and load from the base.

Mouse selection is supported for roster rows, facility cards, action buttons, mission cards, event choices, mission path nodes, combat cards, and the end-turn button.

## Build And Run

```powershell
cargo run
```

Useful checks:

```powershell
cargo fmt --check
cargo check
cargo test
```

## Publishing

Use the project wrapper to call the shared RustGames publisher:

```powershell
.\publish.ps1
```

Generated build outputs belong in ignored directories such as `target/` and `dist/`. Runtime logs and temporary generated image batches are also ignored and can be regenerated when needed.

## Project Layout

- `src/`: game code and state machines.
- `assets/`: shipped JSON data and runtime images.
- `gdd.md`: game design notes.
- `tech_spec.md`: architecture notes.
- `ui_redesign_plan.md`: command-table UI redesign plan and priorities.
- `roadmap.md`: completed roadmap checklist.
- `generate_assets.ps1`, `comfyui-*.ps1`: optional local asset-generation tooling.
