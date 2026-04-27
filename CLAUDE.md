# Project: Metroidvania (Rust + Bevy)

A 2D metroidvania built in Rust with Bevy. Learning Rust gamedev is an explicit goal of the project — this informs decisions where "do it yourself in code" is preferred over "find a plugin," within reason.

## Rules

Detailed rules live in `.claude/rules/`. Read the relevant file when working in that area.

- **[stack.md](.claude/rules/stack.md)** — Engine, version policy, physics approach, level editor, project structure, module layout.
- **[simulation.md](.claude/rules/simulation.md)** — Fixed 60Hz timestep, render-time interpolation, schedule discipline.
- **[platforming-feel.md](.claude/rules/platforming-feel.md)** — Jump mechanics (variable height, coyote, buffer, asymmetric gravity), horizontal movement, wall slide + jump.
- **[input.md](.claude/rules/input.md)** — `leafwing-input-manager`, action set, default bindings.
- **[art-and-render.md](.claude/rules/art-and-render.md)** — 384×216 / 16×16, free asset packs, animation state machine, pixel-perfect rendering.
- **[world.md](.claude/rules/world.md)** — Room transitions, camera, death and respawn, combat resources.
- **[debug.md](.claude/rules/debug.md)** — Inspector, collider gizmos, custom overlays, F1/F12 toggles.
- **[testing.md](.claude/rules/testing.md)** — Inline `#[cfg(test)] mod tests`, Given/When/Then bodies, test pure functions, skip ECS plumbing.
- **[open-questions.md](.claude/rules/open-questions.md)** — Decisions deferred until forced by code or scope.

## Code style

Project-specific overrides on top of the harness defaults:

- **Doc comment every `pub` item.** Every `pub fn`, `pub struct`, `pub enum`, `pub trait`, and `pub mod` gets a `///` (or `//!` for modules) doc comment. Standard rustdoc — readable via `cargo doc --open`. This deviates from the harness default of "no comments unless the WHY is non-obvious."
- **Doc comments explain WHY, not WHAT.** The signature already says what the function does. The doc explains the *reason it exists*, the *invariant it upholds*, what *callers need to know* (units, side effects, panics, edge cases, ordering constraints with other systems). Trivial wrappers can be one line; non-obvious items get a paragraph.
- **Private items (`fn`, `struct` without `pub`) follow the harness default** — no comment unless the WHY is non-obvious. Doc-commenting privates is noise.
- **Tests do not need doc comments.** The Given/When/Then body and behavior-named test name from `testing.md` already document them.

Examples:

```rust
/// Sanitizes a leafwing axis read into the `[-1.0, +1.0]` range that the
/// movement math assumes. Out-of-range values would silently overshoot
/// `max_speed` downstream, so we clamp at the boundary.
pub fn clamp_axis(raw: f32) -> f32 { /* ... */ }

/// Player position in world space, owned by the simulation. The visual
/// `Transform` is *not* this — it is lerped from `PreviousPosition` and
/// this value by the interpolation system. Writing `Transform` directly
/// breaks the lerp.
#[derive(Component)]
pub struct Position(pub Vec2);
```

## Decisions log

| # | Decision | Choice | Rule file |
|---|---|---|---|
| 1 | Engine | Bevy (Rust-native, learning Rust gamedev is the goal) | stack |
| 2 | Bevy version policy | Latest stable, accept churn | stack |
| 3 | Physics approach | `avian2d` kinematic — collision queries only, hand-written movement | stack |
| 4 | Level editor | LDtk + `bevy_ecs_ldtk` | stack |
| 5 | Resolution / tile size / art | 384×216 / 16×16 / free asset packs | art-and-render |
| 6 | Jump feel tier | Modern baseline (variable height + coyote + buffer + asymmetric gravity) | platforming-feel |
| 7 | Horizontal feel | Hybrid: ground-tight, air-momentum, full air control. Stick magnitude is a **throttle** — analog tilt sets target speed (Dead Cells / Ender Lilies model), not just a starting strength. | platforming-feel |
| 8 | Timestep | Fixed 60Hz, render-time interpolation | simulation |
| 9 | Wall mechanics | Slide + jump from the start, 6-frame input lockout | platforming-feel |
| 10 | Camera | Room-locked, pixel-perfect, hard snap | world |
| 11 | Project structure | Single crate, plugin-per-feature, defer workspace | stack |
| 12 | Input | `leafwing-input-manager`, keyboard + gamepad from day one | input |
| 13 | Death / respawn | Respawn at last bench. Lose XP + currency; keep items, resources, abilities. HP + FP refill. No shade. Health bar + FP bar combat resources. | world |
| 14 | Scope | Platforming-fundamentals questions only — combat / economy / UI deferred until code exists | — |
| 15 | Room transitions | Edge-touch only, fade-to-black (~0.3s), velocity preserved, doors deferred | world |
| 16 | Animation | Hand-rolled match on state enum, plain spritesheets, state-deriv in `FixedUpdate`, frame-advance in `Update` | art-and-render |
| 17 | Debug tooling | `bevy-inspector-egui` (F12) + `PhysicsDebugPlugin` + custom flags/velocity overlay (F1), ships in release | debug |
| 18 | Version pin (Bevy ecosystem, Apr 2026) | Bevy 0.18, avian2d 0.6, bevy_ecs_ldtk 0.14, leafwing-input-manager 0.20, bevy-inspector-egui 0.36. LWIM published against Bevy 0.18.0-rc.2 — accept the rc-vs-release risk. | stack |
| 19 | Build profile | `dev` opt-level=1 for project code, opt-level=3 for all deps (standard Bevy pattern; deps cached so slow build pays off once). | stack |
| 20 | Import style | Explicit imports only — no `use bevy::prelude::*`. Deviates from Bevy tutorial idiom; cost is per-file translation tax. | stack |
| 21 | Camera scaling (stage 2) | `ScalingMode::FixedVertical { 216 }`. Not pixel-perfect on non-integer window scales; defer render-to-texture upgrade until sub-pixel jitter is visible. | art-and-render |
| 22 | Build approach | Numbered stages tracked as GitHub issues — see https://github.com/majorgilles/test_game/issues. Each stage answers a specific runtime question; do not skip ahead. | — |
| 23 | Test style | Inline `#[cfg(test)] mod tests`. Given/When/Then comment bodies. Test extracted pure functions; do not test ECS plumbing. | testing |
| 24 | Doc comments | Every `pub` item gets a `///` doc comment explaining WHY/invariants/caller contract — not WHAT. Privates follow the harness default (no comment unless non-obvious). | — |
