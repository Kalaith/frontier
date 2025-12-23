# Frontier Kingdom Roadmap

This document outlines the path from current prototype to MVP (Minimum Viable Product) and finally to Release.

---

## Phase 1: The Minimum Viable Product (MVP)
**Goal:** A complete, playable loop where the user can embark on a mission, fight, return, and progress.

### 1.1 Core Combat Loop (Priority: High)
- [x] Basic Turn Structure (Player -> Enemy).
- [x] Card Playing mechanics.
- [x] **Enemy Intent System:** Display what the enemy *intends* to do next turn (Attack, Block, Buff) so the player can react.
- [x] **Dynamic Enemy Spawning:** Load enemies from `enemies.json` instead of hardcoded structs.
- [x] **Status Effects:** Implement Buffs/Debuffs (Vulnerable, Weak, Stun) in `CombatResolver`.
- [x] **Death & Injury:** Correctly handle adventurer death (permadeath) and lasting injuries if they survive at 0 HP (Death's Door mechanic?).

### 1.2 Mission & Exploration (Priority: High)
- [x] Linear Node Traversal.
- [x] **Meaningful Events:** Implement non-combat nodes (Shrines, Treasures, Traps) with choices that affect HP/Stress/Resources.
- [ ] **Map Variety:** Generate maps with branching paths (Slay the Spire style) instead of a purely linear line.
- [ ] **Mission Types:** Implement logic for `Scout` vs `Suppress` missions (different lengths, enemy density).

### 1.3 Kingdom & Management (Priority: Medium)
- [x] Basic Stats (Gold, Supplies).
- [x] **Recruitment:** Ability to recruit new adventurers.
- [x] **Character Identity:** Gendered variants (Male/Female) for all classes with unique portraits.
- [x] **Facilities:**
    - **Infirmary:** Spend Gold/Supplies to build. Unlocks healing.
    - **Chapel/Tavern:** Spend Gold/Supplies to build. Unlocks stress relief.
    - **Guild Hall:** Unlocks recruitment.
- [x] **Economy:** Earning Gold/Supplies from missions and spending them in the base for buildings and recruitment.

### 1.4 Technical & UI
- [x] Save/Load System (Basic).
- [x] **Deck Viewer:** Allow viewing current deck outside of combat.he Base and during Missions.
- [ ] **Tooltips:** Hover over keywords (Stress, Block) to see explanations.

---

## Phase 2: Alpha (Content & Complexity)
**Goal:** Deepen the systems and add variety.

### 2.1 The Stress & Trauma System
- [ ] **Stress Breakpoint:** When Stress hits 100, trigger a *Resolve Check* (Virtue vs Affliction).
- [ ] **Trauma Traits:** Persistent negative traits gained from high stress missions.
- [ ] **Heart Attacks:** Lethal consequences for max stress.

### 2.2 Content Expansion
- [ ] **Enemy Variety:** 10+ Unique Enemies with distinct AI behaviors.
- [ ] **Card Pool:** 30+ Cards (Unlockable via "Knowledge" resource).
- [ ] **Biomes:** Add a second region (e.g., "The Ruins") with unique art and enemies.

### 2.3 Visual & Audio Polish
- [ ] **VFX:** Particle effects for hits, blocks, and buffs. Screen shake on heavy hits.
- [ ] **SFX:** Sound effects for UI clicks, card plays, impacts.
- [ ] **Music:** Ambient tracks for Base vs Combat.

---

## Phase 3: Release Candidate
**Goal:** Balance, bug fixing, and final polish.

### 3.1 Balance & Progression
- [ ] **Economy Tuning:** Ensure resources are scarce but manageable.
- [ ] **Combat Balance:** Tweaking enemy damage vs player mitigation.
- [ ] **Progression Curve:** Ensure the game gets harder as the Kingdom grows (Threat Level system).

### 3.2 Narrative & Meta
- [ ] **Quest Log:** Long-term goals (e.g., "Defeat the Necromancer").
- [ ] **Kingdom Events:** Random events in the Base (Plague, Thieves, Traders).
- [ ] **Ending:** A win condition (e.g., Establishing a permanent citadel).

---

## Immediate Next Steps (To-Do List)
1.  **Status Effects:** [x] Implement Buffs/Debuffs to deepen combat.
2.  **Map Variety:** [ ] Branching paths for exploration.
3.  **Deck Viewer:** [x] View card decks in Base/Mission.
