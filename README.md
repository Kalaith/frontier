# Frontier Kingdom

A dark card-based expedition RPG inspired by **Darkest Dungeon** and **Slay the Spire**, built in Rust with [Macroquad](https://macroquad.rs/).

## 🎮 Overview

Lead a band of adventurers into the unknown wilds. Build your kingdom, recruit heroes, and send expeditions into dangerous territories. Survive combat through tactical card play, manage your party's stress and health, and unlock new regions as you expand your frontier.

## ✨ Features

### Combat
- **Turn-based card combat** with energy management
- **Enemy intent system** - see what enemies plan to do
- **Status effects** - Vulnerable, Weak, Strengthened, Guarded, Stunned
- **Class-specific cards** for Soldiers, Scouts, Healers, and Mystics

### Expeditions
- **Branching mission maps** - choose your path through each expedition
- **Multiple node types**:
  - ⚔️ Combat encounters
  - ❓ Narrative events with choices
  - ⛺ Rest points (heal HP, reduce stress)
  - 💀 Boss encounters
- **Mission types** with different combat frequency:
  - Scout (25% combat, easier enemies)
  - Suppress (60% combat, harder enemies, boss finale)
  - Secure (40% combat)
  - Investigate (20% combat, narrative-heavy)

### Kingdom Management
- **Buildings** - Infirmary, Chapel, Foundry, Guild Hall, Watchtowers
- **Recruitment** - Hire new adventurers with unique stats
- **Unlock conditions** - Buildings unlock access to new regions/missions
- **Party system** - Form parties of up to 4 adventurers

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.70+)

### Build & Run
```bash
# Clone the repository
git clone https://github.com/talast/frontier-kingdom.git
cd frontier-kingdom

# Run in development mode
cargo run

# Build release version
cargo build --release
```

## 🎮 Controls

### Base
| Key | Action |
|-----|--------|
| Tab | Switch focus (Roster ↔ Buildings) |
| ↑/↓ | Navigate |
| Enter | Select/Build |
| F5 | Save game |
| F9 | Load game |

### Mission
| Key | Action |
|-----|--------|
| Space | Advance to next node |
| ←/→ | Choose path at forks |
| 1-3 | Quick path selection |
| Esc | Retreat |

### Combat
| Key | Action |
|-----|--------|
| ←/→ | Select card |
| Space/Enter | Play card |
| Tab | Switch active party member |
| E | End turn |

## 📁 Project Structure

```
frontier_kingdom/
├── src/
│   ├── main.rs          # Entry point
│   ├── game.rs          # Game loop & state machine
│   ├── state/           # Game states (Base, Combat, Mission, etc.)
│   ├── combat/          # Combat system (cards, resolver, units)
│   ├── missions/        # Mission definitions, events, regions
│   ├── kingdom/         # Kingdom management (buildings, roster, party)
│   ├── data/            # Asset loading (enemies, cards)
│   └── save/            # Save/load system
├── assets/
│   ├── missions.json    # Mission definitions
│   ├── enemies.json     # Enemy definitions
│   ├── cards.json       # Card definitions
│   ├── events.json      # Event definitions
│   └── images/          # Character, enemy, card art
└── roadmap.md           # Development roadmap
```

## 🗺️ Roadmap

See [roadmap.md](roadmap.md) for the full development plan.

**MVP Status:** ~90% complete
- ✅ Core combat loop
- ✅ Mission system with branching paths
- ✅ Kingdom management & buildings
- ✅ Save/Load system
- 🔲 Tooltips & polish

## 📜 License

MIT License - see LICENSE for details.

## 🙏 Acknowledgments

- Inspired by [Darkest Dungeon](https://www.darkestdungeon.com/) and [Slay the Spire](https://www.megacrit.com/)
- Built with [Macroquad](https://macroquad.rs/)
