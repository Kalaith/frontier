# Frontier Kingdom – Technical Design Document

This document defines the **technical architecture**, **coding standards**, and **system boundaries** for the Frontier Kingdom card-based expedition RPG.

The goal is sustainability, clarity, and long-term iteration using a **lightweight game framework** (Macroquad), not a monolithic engine.

---

## 1. Technology Stack

### Language
- **Rust**

Chosen for:
- Strong type safety
- Explicit state management
- Excellent performance for simulation-heavy systems
- Clear separation of data and behavior

---

### Framework
- **Macroquad**

Used as:
- Rendering layer
- Input handling
- Audio playback
- Main loop timing

Not used as:
- Scene manager
- Game state authority
- UI framework

Macroquad remains deliberately thin.

---

## 2. Architectural Principles

### Code-First, Data-Driven
- Game rules live in code
- Content lives in data files (JSON / RON)
- Balance changes do not require recompilation

---

### Explicit State Machines
No hidden transitions.
No magic callbacks.

All game flow is driven by explicit state changes.

---

### Separation of Concerns
- Rendering does not contain logic
- UI does not mutate game state directly
- Cards emit effects, systems resolve them

---

## 3. High-Level Project Structure

src/
├─ main.rs // Macroquad entry point
├─ game.rs // Global game loop & state switching
├─ state/
│ ├─ mod.rs
│ ├─ base.rs // Kingdom/base state
│ ├─ mission_select.rs // Mission selection and embark flow
│ ├─ mission.rs // Mission map & flow
│ ├─ event.rs // Narrative mission events
│ ├─ combat.rs // Combat state
│ ├─ recruit.rs // Recruitment state
│ └─ results.rs // Post-mission resolution
│
├─ combat/
│ ├─ mod.rs
│ ├─ unit.rs
│ ├─ card.rs
│ ├─ effects.rs
│ └─ resolver.rs
│
├─ kingdom/
│ ├─ mod.rs
│ ├─ adventurer.rs
│ ├─ party.rs
│ ├─ roster.rs
│ ├─ stats.rs
│ ├─ buildings.rs
│ └─ unlock.rs
│
├─ missions/
│ ├─ mod.rs
│ ├─ mission.rs
│ ├─ region.rs
│ └─ events.rs
│
├─ ui/
│ └─ mod.rs
│
├─ data/
│ ├─ mod.rs
│ ├─ cards.rs
│ └─ enemies.rs
│
└─ save/
  └─ mod.rs

assets/
├─ cards.json
├─ enemies.json
├─ missions.json
├─ regions.json
├─ *_prompts.json
└─ images/

yaml
Copy code

---

## 4. Game State Management

### Global Game State Enum

```rust
enum GameState {
    Base(BaseState),
    MissionSelect(MissionSelectState),
    Mission(MissionState),
    Combat(CombatState),
    Results(ResultState),
    Event(EventState),
    Recruit(RecruitState),
}
Only one active state at a time

Transitions are explicit

No shared mutable global state

5. Combat System Architecture
Core Rule
Cards do not directly mutate units.

They emit effects.

Card Model
rust
Copy code
struct Card {
    id: String,
    cost: i32,
    effects: Vec<CardEffect>,
}
Effects
rust
Copy code
enum CardEffect {
    Damage(i32),
    Block(i32),
    Stress(i32),
    ApplyStatus { status: Status, stacks: i32 },
}
Resolution Flow
css
Copy code
Player selects card
 → Card emits effects
 → Resolver validates legality
 → Effects applied in order
 → Triggers processed
 → State updated
This allows:

Replays

Logging

AI simulation

Easy balance changes

6. Stress & Trauma System
Stress is stored on units and persists between states.

Stress Thresholds
Thresholds are data-driven

Crossing a threshold triggers trauma checks

Trauma modifies unit behavior and card interactions

Trauma resolution occurs outside combat, usually in base systems.

7. Data-Driven Content
All content is defined in external files.

Examples
Cards

Enemies

Regions

Mission templates

Events

This enables:

Fast iteration

Modding potential

Balancing without recompilation

8. UI Philosophy
Immediate Mode UI
Buttons, panels, text drawn every frame

Stateless rendering

Input polling per frame

Rules
UI reads state

UI emits intents

Game logic applies changes

No logic hidden inside UI callbacks.

9. Save & Load System
Save Scope
Kingdom state

Adventurer roster

Region progress

Decks, injuries, trauma

Strategy
Serialize a single SaveData struct

Versioned saves

Human-readable format preferred (RON / JSON)

10. MVP Technical Scope
Phase 1
State machine

Basic combat

Static card set

One region

Manual save/load

Phase 2
Stress & trauma

Mission events

Base upgrades

Expanded card pool

Phase 3
AI behaviors

Region progression

Difficulty scaling

Mod hooks

11. Non-Goals (Important)
No editor tooling

No ECS overengineering

No real-time combat

No procedural generation until core systems are stable

Simplicity is a feature.

12. Long-Term Flexibility
This architecture supports:

New card types

New regions

New factions

Alternate game modes

Balance patches without refactors
