# Frontier Kingdom: Card-Based Expedition RPG

A dark, systems-driven RPG inspired by **Darkest Dungeon** and **Slay the Spire**, built as a **code-first game** using a lightweight framework (Macroquad).  
You manage a fragile frontier kingdom, sending adventurers into hostile wilds, learning through failure, and shaping what kind of kingdom survives.

---

## 1. Core Fantasy

You are not a hero.

You are the authority that sends others into danger.

The kingdom is unfinished, under-resourced, and surrounded by threats that do not want to be tamed.  
Progress is uneven. Survival is costly. Victory leaves scars.

---

## 2. Core Gameplay Loop

Kingdom Base
↓
Prepare Adventurers + Decks
↓
Mission / Expedition
↓
Card-Based Combat & Events
↓
Stress, Injuries, Death
↓
Return or Collapse
↓
Spend Resources / Build / Unlock
↓
Repeat

yaml
Copy code

Failure is expected and feeds progression.

---

## 3. The Frontier Kingdom (Meta Layer)

The kingdom is a collection of **systems**, not flavor.

### Core Kingdom Stats
- **Security** – Road safety, encounter predictability
- **Morale** – Willingness to send people out
- **Supplies** – Equipment and expedition readiness
- **Knowledge** – Understanding enemies and regions
- **Influence** – How other factions respond

These stats pull against each other. There is no perfect state.

---

## 4. Adventurers

Adventurers are persistent resources that remember.

Each has:
- HP and Stress
- Traits (background, quirks)
- Injuries and Trauma (persistent)
- A personal card deck

Veterans are stronger but more fragile.  
No one remains clean forever.

Death matters:
- It affects morale
- Unlocks new cards or events
- Changes surviving adventurers

---

## 5. Stress & Trauma

Stress is a **meta-pressure system**.

- Gained from combat, events, and risky choices
- Persists after missions
- Crossing thresholds triggers **Trauma**

Trauma examples:
- Increased card costs
- Skipped turns
- Reduced healing
- New negative traits

Stress can be reduced, but always at an opportunity cost.

---

## 6. Expeditions & Missions

Expeditions are contracts with uncertainty.

### Mission Types
- **Scout** – Low danger, knowledge-focused
- **Suppress** – Combat-heavy, stabilizes regions
- **Secure** – Enables trade or settlement
- **Investigate** – Narrative events, high stress

### Mission Flow
Departure
→ Route Event
→ Combat / Choice
→ Escalation
→ Extraction or Collapse

yaml
Copy code

Longer missions escalate danger and stress.

---

## 7. The Wilds

Regions are not conquered, only stabilized.

Each region has:
- Threat level
- Unknown traits
- A defining horror or faction

Taming a region makes travel safer but creates new problems:
- Displaced enemies
- Rival factions
- Internal political pressure

---

## 8. Base Building

Buildings unlock **options**, not raw power.

Examples:
- **Infirmary** – Heal injuries
- **Chapel** – Reduce stress, increase tension
- **Foundry** – Upgrade cards and gear
- **Watchtowers** – Safer routes, stronger enemies
- **Guild Hall** – Specialists, internal politics

You cannot build everything early. Choices matter.

---

## 9. Cards as the Core Combat System

Combat is turn-based and card-driven.

- Cards emit effects
- Combat systems resolve effects
- Cards never directly mutate state

Cards represent:
- Training doctrines
- Cultural beliefs
- Hard-earned lessons

New cards are unlocked through:
- Surviving enemies
- Losing expeditions
- Kingdom development

---

## 10. Starter Deck – First 10 Cards

These are Tier 0 foundational cards.  
Every card introduces a future design axis.

### 1. Strike
- **Cost:** 1  
- **Effect:** Deal 6 damage

Baseline reference card.

---

### 2. Guard
- **Cost:** 1  
- **Effect:** Gain 5 Block

Basic survival pacing.

---

### 3. Focused Blow
- **Cost:** 1  
- **Effect:** Deal 4 damage  
  If the target has no Block, deal +3 damage

Introduces conditional damage.

---

### 4. Brace
- **Cost:** 1  
- **Effect:** Gain 3 Block  
  Reduce incoming Stress this turn by 50%

Stress mitigation as a tactical choice.

---

### 5. Desperate Swing
- **Cost:** 0  
- **Effect:** Deal 5 damage  
  Gain 5 Stress

Free actions always have a cost.

---

### 6. Measured Strike
- **Cost:** 2  
- **Effect:** Deal 10 damage

Tempo control through higher commitment.

---

### 7. Recenter
- **Cost:** 1  
- **Effect:** Reduce Stress by 6

Stress management trades momentum.

---

### 8. Opportunistic Cut
- **Cost:** 1  
- **Effect:** Deal 5 damage  
  If the enemy acted last turn, deal +4 damage

Turn order awareness.

---

### 9. Hold the Line
- **Cost:** 2  
- **Effect:** Gain 8 Block  
  Cannot play Attack cards this turn

Defense locking out aggression.

---

### 10. Last Ditch Effort
- **Cost:** 1  
- **Effect:** Deal 8 damage  
  If HP is below 30%, deal +6 damage  
  Gain 3 Stress

Clutch moments that leave scars.

---

## 11. Opening 10 Minutes (Player Experience)

- The kingdom is introduced as unfinished and fragile
- The player meets a small, unideal roster
- First mission presents risk with no perfect option
- Stress is introduced before combat
- First combat teaches that stress matters as much as HP
- Aftermath shows consequences carrying home

The game teaches through systems, not tutorials.

---

## 12. Design Pillars

- Systems over spectacle
- Failure as progression
- Persistent consequences
- Data-driven design
- Framework-first, engine-free architecture

---

## 13. Long-Term Direction

This is not an empire builder.

The endgame is deciding:
- what the kingdom sacrifices
- what it protects
- and what it refuses to become

A stable kingdom is a victory.