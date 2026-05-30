# Frontier Kingdom UI Redesign Plan

## Design Target

Frontier Kingdom should feel like dark fantasy expedition management from a candlelit command table. The player is not walking around a town or operating a spreadsheet. They are reading reports, weighing debts, checking wounds, interpreting omens, and deciding which exhausted heroes go back into the woods.

The UI should be menu-driven, but each menu should feel like a physical layer on the table: maps, reports, contracts, tokens, ledgers, grave records, bandages, and sealed orders.

## Core Navigation Model

Use top-level tabs to give each management mode a strong identity without building a 2D town hub:

- Kingdom
- Roster
- Missions
- Buildings
- Deck / Training
- Graveyard
- Journal

The town/base screen should answer three questions immediately:

- Who is ready?
- What can I afford?
- Where should I go next?

## Visual Direction

Primary fantasy: a candlelit command table.

Recommended background:

- Top-down or angled table view with maps, coins, sealed letters, wooden markers, wax seals, bloodied bandages, ink, and guttering candlelight.
- Dark edges with warmer center illumination.
- No navigable town map unless building placement later becomes mechanical.

Palette direction:

- Normal text: off-white, parchment, grey, dull brass.
- Ready/safe: muted moss.
- Selected/reward: candle gold.
- Danger/blocked: blood red.
- Information/scouting: cold steel.
- Mystery/trauma: occult violet.

Bright colors should be reserved for selected state, ready action, danger, rewards, and blocked requirements.

## Town Screen Redesign

### First-Pass Layout

```text
FRONTIER KINGDOM
Day 1 · Threat 1 · Morale Fragile

┌───────────────┬────────────────────────────┬─────────────────┐
│ RESOURCES     │ ADVENTURERS                │ CURRENT GOALS   │
│ Gold 120      │ Marcus       Ready         │ Scout Woods     │
│ Supplies 65   │ Elena        Stressed      │ Build Towers    │
│ Security 30   │ Aldric       Needs Rest    │ Clear Outpost   │
│ Knowledge 10  │                            │                 │
├───────────────┴────────────────────────────┴─────────────────┤
│ KINGDOM ACTIONS                                               │
│ [Embark] [Roster] [Facilities] [Treat] [Recruit] [Save]       │
├───────────────────────────────────────────────────────────────┤
│ DETAILS                                                       │
│ Selected adventurer, facility, quest, or mission appears here.│
└───────────────────────────────────────────────────────────────┘
```

The top half is a dashboard. The bottom half is contextual.

The dashboard should show only summary information:

- Resources and affordability signals.
- Adventurer readiness, not full stats.
- Current goals and alerts.

The details panel should change based on selection:

- Selected adventurer: stats, trauma, injuries, role guidance, and actions.
- Selected facility: purpose, cost, status, unlocks, and action button.
- Selected quest: requirement, destination, reward, and next step.
- Selected mission: risk, reward, party readiness, and embark action.

### Selected Adventurer Detail Panel

Required fields:

- Name
- Class and level
- HP
- Stress
- Injuries
- Traits / trauma
- Deck size
- Best use
- Actions

Example:

```text
MARCUS
Soldier · Level 1

HP: 45/45
Stress: 10
Injuries: None
Trauma: None
Deck: 8 cards

Best Use:
Strong frontline fighter. Best for Suppress and Combat-heavy missions.

Actions:
[View Deck] [Assign to Party] [Rest] [Train]
```

### Selected Facility Detail Panel

Required fields:

- Facility name
- Built/not built status
- Purpose
- Cost or active use
- Unlocks
- Primary action

Example:

```text
INFIRMARY
Not Constructed

Purpose:
Treat wounds before injuries become permanent.

Build Cost:
50 Gold, 20 Supplies

Unlocks:
- Treat Wounds
- Reduce injury death risk
- Later: Surgeon upgrade

[Build Infirmary]
```

## Buildings As Facility Cards

Replace the vertical cost list with facility cards. Cards should communicate purpose before cost.

Facility purposes:

| Facility | UI Purpose |
| --- | --- |
| Infirmary | Heal injuries before they become permanent. |
| Chapel | Reduce stress and prevent resolve collapse. |
| Foundry | Improve equipment and unlock stronger cards. |
| Guild Hall | Recruit, dismiss, and train adventurers. |
| Watchtowers | Lower threat and unlock scouting missions. |
| Citadel | Final objective and win condition. |

Each card should include:

- Name
- Purpose line
- Cost, if not built
- Status
- Available actions

Example:

```text
┌──────────────────────┐
│ INFIRMARY            │
│ Treat injuries       │
│ Cost: 50g / 20s      │
│ Status: Not built    │
│ [Build]              │
└──────────────────────┘
```

## Mission Board And Embark Preparation

The mission screen should become the pre-expedition tension screen.

Mission cards should remain readable even when locked. Locked cards teach the player what to do next.

### Selected Mission Panel

```text
MISSION: Scout the Dark Woods

Party:
Marcus    HP 45/45    Stress 10    Risk: Low
Elena     HP 35/35    Stress 10    Risk: Low

Expected:
Difficulty: 1
Stress Gain: 8
Region: Dark Woods
Possible Encounters: Beasts, Events, Unknown

Rewards:
25 Gold
10 Supplies
15 Knowledge

Warnings:
None

[Embark] [Change Party] [Back]
```

Locked mission copy:

```text
LOCKED
Requires: Watchtowers
Build Watchtowers to scout beyond the ruined road.
```

## Expedition Map Polish

The mission map is close to the target mood because it already has background art and route tension. Improvements should make node language clearer.

Node icons:

| Node Type | Suggested Icon |
| --- | --- |
| Unknown | ? |
| Combat | crossed blades or red eye |
| Event | scroll or candle |
| Rest | campfire |
| Treasure | chest |
| Boss | skull or crown |

Add a legend:

```text
? Unknown   ⚔ Combat   ✦ Event   ☾ Rest   ☠ Boss
```

Anchor party portraits in a small panel:

```text
Expedition Party
Marcus   HP 45/45
Elena    HP 35/35
```

## Combat Screen Redesign

The combat screen should use the center as a dramatic battlefield and decision preview area.

Suggested structure:

```text
┌─────────────────────────────────────────────┐
│ Enemy Area                                  │
│              Forest Beast                   │
│              HP 30/30                       │
│              Intent: Attack 8               │
│                                             │
├─────────────────────────────────────────────┤
│ Battle Log / Effects / Status Preview       │
│ Marcus will take 8 damage unless blocked.   │
├─────────────────────────────────────────────┤
│ Player Area                                 │
│ Marcus HP 45/45 Block 0 Stress 10 Energy 3 │
│ [Card] [Card] [Card] [Card] [Card]          │
└─────────────────────────────────────────────┘
```

Priority changes:

- Move enemy presentation closer to center.
- Make enemy intent larger and more readable.
- Make active adventurer clearer.
- Use the middle band for battle log, incoming damage preview, and hovered-card preview.
- On card hover, show expected result:

```text
Strike
Deal 6 damage to Forest Beast.
Enemy HP after: 24/30.
```

## Card Template Direction

Card art can remain temporary. Card frames should become consistent.

Required card template:

```text
┌────────────────────┐
│ Cost       Type    │
│                    │
│       Artwork      │
│                    │
│ Card Name          │
│ Effect text        │
└────────────────────┘
```

Frame categories:

| Type | Frame Feel |
| --- | --- |
| Attack | iron with blood-red accent |
| Guard | steel with blue-grey accent |
| Heal | pale green and candle-gold accent |
| Skill | leather and brass accent |
| Mystic | black iron and occult violet accent |

Avoid relying on text baked into artwork. All readable card information should be rendered by the game UI.

## Keyboard Command Clutter

Keep keyboard shortcuts, but make primary actions visible as buttons or tabs.

Base screen target:

```text
Actions:   [Embark]  [Roster]  [Facilities]  [Decks]  [Save]
Shortcuts: 1-9 Select · Tab Focus · Esc Back
```

The player should see clear actions first and shortcuts second.

## Implementation Phases

### Phase 1: Town UX

Highest impact because the town/base screen is currently the least atmospheric and most list-heavy.

Tasks:

- Add top-level tabs.
- Add candlelit command-table background.
- Rebuild base layout around resources, readiness, goals, actions, and contextual details.
- Convert buildings into facility cards.
- Add selected adventurer detail panel.
- Add selected facility detail panel.
- Make locked/unavailable items readable.
- Replace command-heavy footer with visible actions plus smaller shortcut text.

Acceptance criteria:

- Base screen immediately answers who is ready, what is affordable, and where to go next.
- Every selected item has a detail panel.
- Every facility communicates purpose, status, and action.
- Normal text is restrained; bright colors are rare and meaningful.

### Phase 2: Combat Clarity

Tasks:

- Center enemy presentation.
- Enlarge enemy intent.
- Add battle log/effects preview band.
- Add hovered-card result preview.
- Standardize card frame rendering.

Acceptance criteria:

- The player can understand current incoming danger in under two seconds.
- Hovering a card communicates concrete expected impact.
- Cards share a coherent frame style regardless of art source.

### Phase 3: Expedition Polish

Tasks:

- Improve node icons.
- Add node legend.
- Add anchored expedition party panel.
- Improve mission result screen with injuries, stress, loot, and kingdom event summary.

Acceptance criteria:

- Node meanings are understandable without trial and error.
- Party status is anchored and readable.
- Mission results tell a short, useful story of cost and reward.

## Non-Goals

- Do not build a 2D town map unless physical placement becomes a mechanic.
- Do not hide keyboard support; just stop making shortcut text the primary UI.
- Do not chase perfect card art before the card frame and layout are consistent.
- Do not overhaul every screen in one pass.

## Strong Recommendation

Keep the fantasy focused:

The player is managing a doomed frontier charter from a desk covered in maps, blood, debt, and bad weather.
