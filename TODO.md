# TODO — Frontier Kingdom

## Shell and navigation

- There is no title screen; the game drops straight into the base. Add Continue / New Game / Settings / Exit.
- Every screen still draws a `Shortcuts:` footer (`base/panels.rs`, `combat/draw.rs`, `mission_select.rs`). Keep the keys, demote the text.
- Some screens are not yet fully mouse-driven; a few actions remain keyboard-only.
- The base action bar duplicates entries already on the top tab bar — keep one.

## Screens

- A few screens still use the pre-redesign layout and have overlapping elements; `src/state/mission_select.rs` is the largest holdout at 771 lines.
- Combat needs more feedback on hits, blocks, and status changes — the mechanics read fine but land quietly.

## Content

- No tutorial or framing narrative; a short guided opening would carry the player into the emergent loop.

## Testing

- The crate has no tests. Start with mission-state selection/launch/resolution/reward/failure paths.
- Extract recruit, event, and combat reward math into pure evaluators with fixtures for low-resource and over-capacity cases.
- Add campaign fixtures covering base upgrades, mission chains, kingdom events, and result-screen progression.
- Consolidate state-screen navigation so base, combat, event, recruit, and results share one transition policy.
