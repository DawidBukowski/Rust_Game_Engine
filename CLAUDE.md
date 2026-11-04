# Isometric Engine — Architectural Rules & Protocol

## STOP-REVIEW-COMMIT Protocol
1. **FORBIDDEN:** `git commit`, `git push`, `git merge`, `git rebase`, `git reset` via CLI tools. The human developer performs all git operations manually.
2. **One Milestone at a Time:** Never implement more than one milestone per turn.
3. **Self-Verification Before Presenting:**
   ```bash
   cargo check --all-targets
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt --check
   ```
4. **HARD STOP** after each milestone summary. Wait for human approval before proceeding.

## ECS Architecture Rules
- **Components** (`components.rs`): Pure data structs/enums. Zero business logic, zero cross-entity mutation.
- **Systems** (`systems.rs`): Small, single-responsibility functions with tightly scoped queries.
- **Events**: Cross-domain communication flows through strongly-typed `Event` structs. Modules never reach into another domain's internals.

## SystemSet Ordering (FixedUpdate at 60Hz)
```
InputSet → IntentSet → MovementPhysicsSet → CombatStateSet → HitDetectionSet → DamageAndBoonSet → AiSet → CleanupSet
```

## Module Boundaries
| Module | Path | Responsibility |
|--------|------|---------------|
| `core` | `src/core/` | FixedUpdate pipeline, SystemSets, simulation time control |
| `spatial` | `src/spatial/` | Camera rig, ground-plane raycasting |
| `input` | `src/input/` | Input mapping, action intents, ring-buffer queue |
| `combat` | `src/combat/` | State machine, hitbox/hurtbox/pushbox, hit resolution |
| `boons` | `src/boons/` | Stat pipeline, trigger/condition/effect, status effects |
| `ai` | `src/ai/` | Enemy behavior FSM, armor/poise, attack token director |
| `dungeon` | `src/dungeon/` | Chamber lifecycle, wave spawner, run graph |
| `debug` | `src/debug/` | F1 gizmos, F2 egui overlay, sandbox tools |

## Code Quality Rules
- **Max 300 lines per `.rs` file.** Split by responsibility.
- **No `.unwrap()` / `.expect()` in runtime ECS systems.** Use `let Ok(..) = .. else { return; };` or `Option` combinators with `warn!`.
- **No hardcoded gameplay numbers.** All values in `assets/config/*.ron` files, loaded as Bevy Resources.
- **Headless tests** with `MinimalPlugins` for every milestone.
- **Explicit system ordering** via `SystemSet` — never rely on implicit order.

## Key Events (cross-domain)
```
ActionRequestedEvent, DashStartedEvent, HitConfirmedEvent,
DamageAppliedEvent, StatusAppliedEvent, EnemyKilledEvent,
ArmorBrokenEvent
```

## Tech Stack
- **Bevy 0.20.1** + **bevy_egui 0.43.1**
- **serde** + **ron** for data-driven config
- **60Hz FixedUpdate** deterministic simulation (frame data in `u32` ticks)
