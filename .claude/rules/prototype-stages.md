# Prototype build stages

The platforming prototype is built in numbered stages. Each stage is a verifiable runtime milestone — when it compiles and runs, it answers a specific question. **Do not skip ahead** before the previous stage is verified, because the value of staging is catching version-pin and architectural problems early when only one variable changed.

## Stage 1 — Stack compiles cleanly

**Goal:** prove the version pin is coherent. All five committed crates (`bevy`, `avian2d`, `bevy_ecs_ldtk`, `leafwing-input-manager`, `bevy-inspector-egui`) are listed in `Cargo.toml`. Only `bevy` is actually used.

**Pass criteria:** `cargo check` completes without errors.

## Stage 2 — Window + placeholder square (current)

**Goal:** prove Bevy renders to a window and a sprite is visible at the agreed internal resolution.

**Implemented:**
- `DefaultPlugins` registered.
- Nearest-neighbor texture filter at the app level (`ImagePlugin::default_nearest`) — required for pixel art, set globally so it can't be forgotten per-texture later.
- 2D camera with `ScalingMode::FixedVertical { viewport_height: 216.0 }` so 216 world units fill the window vertically; horizontal extent stretches with aspect ratio.
- Single 16×16 white sprite at world origin (0, 0, 0) — placeholder for the player.

**Pass criteria:** `cargo run` opens a window with a small white square at center.

**Deliberately deferred (do NOT add until the relevant later stage):**

- **`Player` marker component** — added in stage 3 when there's actual movement state to attach.
- **Plugin-per-feature module structure** — current `main.rs` registers one `Startup` system directly. Plugin extraction happens in stage 3 alongside the player module.
- **Z-layering convention** — only one sprite exists; no layering question yet. Convention to be defined when a second sprite type appears.
- **Pixel-perfect render-to-texture pipeline** — `FixedVertical` is *not* truly pixel-perfect on non-integer-divisible window sizes (e.g., 1366×768 → 3.555× scale). Acceptable for prototyping. Upgrade to render-to-texture pipeline only when sub-pixel jitter becomes visible in pixel art with detail.
- **`avian2d`, `bevy_ecs_ldtk`, `leafwing-input-manager`, `bevy-inspector-egui` plugins** — listed in `Cargo.toml` (so version mismatches surface now) but their plugins are *not* registered with the `App`. Each is registered in the stage that uses it.
- **Window title, size, vsync settings** — Bevy defaults are fine; customize when product polish matters.
- **`use bevy::prelude::*`** — explicit imports only, by project convention. Every Bevy file in this repo enumerates its imports; the prelude glob is rejected. This deviates from Bevy tutorial style and means tutorial code requires translation.

## Stage 3 — Walking square (planned)

**Goal:** prove leafwing input plumbing and the `FixedUpdate` schedule work, with input edges (`just_pressed`/`just_released`) observable per fixed tick.

**Will add:**
- `leafwing-input-manager` plugin registered.
- `PlayerAction` enum and `InputMap` (per `input.md`).
- `Player` marker component.
- Plugin-per-feature module split: `src/player/mod.rs`, `src/player/movement.rs`.
- Horizontal movement only — ground accel/decel curves from `platforming-feel.md`. No gravity, no jump, no ground (the square just slides on an invisible surface).

**Will *not* yet add:** physics colliders, gravity, jump, animation, levels.

## Stage 4 — Walking + colliding (planned)

**Goal:** prove `avian2d` collision queries work for our hand-written-movement model. A static ground rectangle stops the player from falling through it.

**Will add:**
- `avian2d` plugin registered.
- Static ground entity with a `Collider`.
- Gravity (constant, *not* yet asymmetric — that's stage 5).
- Ground detection via shape-cast or raycast.

## Stage 5 — Modern-baseline jump (planned)

**Goal:** the jump feels right.

**Will add (in this exact order, validate feel between each):**
1. Basic jump impulse on `just_pressed(Jump)`.
2. Variable jump height (cut velocity on `just_released(Jump)` if rising).
3. Coyote time (~6 fixed ticks of grace after leaving ground).
4. Jump buffer (~6 fixed ticks of grace before landing).
5. Asymmetric gravity (~1× rising, ~1.5–2× falling).

Tuning numbers come from `platforming-feel.md`. Values are starting points; tune against the running prototype.

## Stage 6+ — TBD

Wall mechanics, LDtk loading, room transitions, animation, debug overlays. Order to be decided when stage 5 is shipped.
