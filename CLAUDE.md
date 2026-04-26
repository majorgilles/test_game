# Project: Metroidvania (Rust + Bevy)

A 2D metroidvania built in Rust with Bevy. Learning Rust gamedev is an explicit goal of the project — this informs decisions where "do it yourself in code" is preferred over "find a plugin," within reason.

## Foundational stack

- **Language:** Rust
- **Engine:** Bevy, latest stable. Accept the per-release API churn as part of learning. If a new Bevy version drops in the same week as starting work, wait 2–3 weeks for the third-party plugin ecosystem to catch up before upgrading.
- **Physics:** `avian2d` in **kinematic / collision-query mode only**. Movement math (velocity, gravity, jump arc) is hand-written. `avian2d` is used for `Collider` shapes, raycasts, and shape-casts to answer "what did I hit." We do *not* use its rigid-body dynamics — platformer feel needs hand-tuned movement, not realistic physics.
- **Levels:** **LDtk** (ldtk.io) authored, loaded via `bevy_ecs_ldtk`. Chosen over Tiled because LDtk's data model — many "levels" (rooms) arranged spatially in a "world," typed entity instances with strongly-typed fields — matches metroidvania structure directly and avoids writing a custom world-graph layer.
- **Target resolution:** **384×216 internal** (16:9), **16×16 tiles**. Renders to a fixed-size canvas and is upscaled (integer scale) to the window — ×5 to 1080p, ×10 to 4K. No blurry scaling, no letterbox math.
- **Art source:** Free asset packs (Kenney, itch.io, OpenGameArt) while the focus is on code. We are not drawing original art at this stage.

## Simulation model

- **Fixed-timestep physics at 60 Hz.** All movement, collision, gravity, jump-cutoff, coyote-time, and jump-buffer logic runs in Bevy's `FixedUpdate` schedule. "N frames" of coyote/buffer means N fixed steps regardless of monitor refresh rate.
- **Render-time interpolation.** Sprites' `Transform` is lerped between `previous_position` and `current_position` by `fixed_time.overstep_fraction()` so visuals are smooth on 120/144/240Hz monitors despite physics ticking at 60Hz.
- **Why fixed:** determinism across hardware, framerate-independent feel, and timing-sensitive mechanics (variable jump cutoff, coyote time) only mean something when a "frame" is a fixed duration.

## Platforming feel

### Jump — modern baseline
Implementation order: each addition should be a felt improvement before moving to the next.

1. **Variable jump height.** Hold to jump higher; release early to cut the jump short. Implemented by setting `velocity.y *= 0.5` (or similar) the frame the jump button releases *if* `velocity.y > 0`.
2. **Coyote time.** ~6 frames of grace after walking off a ledge during which a jump press still counts as a grounded jump. Tracked via a `coyote_timer` that resets on grounded state and counts down while airborne.
3. **Jump buffer.** ~6 frames of grace where a jump press *just before* landing is queued and consumed on landing. Tracked via a `jump_buffer_timer` that is set on jump press and counts down each tick.
4. **Asymmetric gravity.** Lower gravity going up (~1× base), higher going down (~1.5–2× base). Optionally a third, even higher value for "fast fall" if down is held while airborne — to be decided later.

We are **not** implementing apex hangtime, dynamic friction tuning, or jump-corner-correction at this stage. They are correctly tuned only against real level geometry, not in a vacuum.

### Horizontal — hybrid, ground-tight / air-momentum

- Ground accel: reach max speed in ~4 frames (~0.07s)
- Ground decel: stop from max speed in ~5 frames (~0.08s)
- Air accel: reach max speed in ~10 frames (~0.17s) — roughly half ground accel
- Air decel: ~12 frames
- Max speed: ~6–8 tiles/second → ~96–128 px/s at 16-px tiles
- **Full air control** (player can press the opposite direction mid-jump), but the reduced air accel means direction reversal is not instant — jumps feel committed without being locked.

These numbers are **starting values**, not final. They will be tuned against actual level geometry once a test room exists.

## Wall mechanics

Wall slide + wall jump available **from the start** (not gated behind an upgrade at this prototyping stage).

- **Wall slide.** When airborne, moving into a wall, and `velocity.y < 0`, clamp fall speed to a slow max (~60 px/s).
- **Wall jump.** On jump press while wall-sliding, `velocity = (-wall_normal * kickoff_x, jump_velocity_y)`. Variable jump height applies (same vertical impulse as ground jump, same release-cut behavior).
- **Input lockout.** ~6 frames of horizontal input lockout after kickoff so the player can't immediately re-stick to the same wall.
- Full wall climb (free vertical movement on walls) is **not** available — that's reserved as a potential late-game upgrade.

## Camera

Start simple, add complexity only when a room demands it.

- **Room-locked.** Camera position = current room's center, in world space. One room fills the viewport.
- **Pixel-perfect.** Camera translation is rounded to whole pixels each frame after positioning, otherwise sprites jitter against the tilemap due to subpixel sampling — non-negotiable for pixel art.
- **Hard snap on transition.** No smooth blend between rooms (yet). Add a blend later only if the snap feels jarring.
- **No deadzone scrolling yet.** When we hit a room genuinely too large for one viewport, add deadzone-based smooth-follow as a *second* camera mode. Until then, room-locked is enough.

## Project structure

- **Single crate.** `Cargo.toml` at the project root, all code under `src/`. Defer workspace splitting until compile times measurably hurt; Bevy compile time is dominated by monomorphization of generic ECS code, not by crate count, so splitting is rarely the right lever early.
- **Compile-time mitigations** (apply when needed, not preemptively): `bevy/dynamic_linking` feature in dev profile, `lld` linker on Windows, `cargo-watch` for tight loops.
- **Module layout** — each subdirectory is a Bevy `Plugin`, composed in `main.rs`:

```
src/
  main.rs           App setup, plugin registration
  player/
    mod.rs          Player plugin
    movement.rs     Horizontal accel/decel, ground vs. air
    jump.rs         Jump, coyote, buffer, variable height, asymmetric gravity
    wall.rs         Wall slide, wall jump
  physics/
    mod.rs          Collision query helpers, ground/wall detection
    interpolation.rs Render-time Transform interpolation
  level/
    mod.rs          LDtk loading, room transitions
  camera/
    mod.rs          Room-locked camera, pixel-perfect snap
  input.rs          Input mapping (likely leafwing-input-manager)
```

## Input

`leafwing-input-manager` (LWIM) for action-based input. Keyboard + gamepad supported from day one.

- Game systems read **named actions** (`PlayerAction::Jump`), never raw `KeyCode` or `GamepadButton`. Bindings live in a single `InputMap` attached to the player entity.
- LWIM exposes `pressed` (level — is it down now?), `just_pressed` (rising edge — went down this frame), and `just_released` (falling edge — went up this frame) per action. The full triple is required: `pressed` for continuous effects (wall slide, hold-to-move), `just_pressed` for discrete impulses (start jump, swing attack), `just_released` for terminating events (cut jump short for variable height).
- The `Move` action is an **axis** (`f32` in `[-1.0, +1.0]`) so analog stick magnitude carries through to movement systems — enables proportional walking from a half-tilted stick without per-system axis blending.

**Starting action set & default bindings:**

| Action | Type | Keyboard | Gamepad |
|---|---|---|---|
| `Move` | axis | A/D, ←/→ | Left stick X |
| `Jump` | button | Space | South (A / X) |
| `Attack` | button | (TBD) | (TBD) |
| `Dash` | button | (TBD) | (TBD) |
| `Interact` | button | (TBD) | (TBD) |
| `Pause` | button | Escape | Start |

`Attack` / `Dash` / `Interact` bindings are placeholders — set them when those mechanics get implemented.

**Risk mitigation.** LWIM occasionally lags Bevy releases. At `Cargo.toml` time, verify there is a published LWIM version compatible with the Bevy version we pin. If not, pin Bevy one minor version back rather than fall back to raw input — the ergonomic delta is large enough that staying current on LWIM is worth a brief Bevy delay.

## Death and respawn

Hollow-Knight-lite stakes: death stings but doesn't gate progress.

**On death:**
- Fade out (~0.5s), then respawn at the position of the last activated bench. If no bench activated yet, respawn at world spawn.
- **Lost:** accumulated XP, accumulated currency.
- **Kept:** acquired items (keys, ability shards, tools), resources (consumable materials, crafting components — collected inventory items), abilities/upgrades.
- **Restored:** Health bar to max, FP bar to max.
- **No shade / corpse-run.** Lost XP and currency are simply gone; no entity persists at the death location.

**On respawn (room state):**
- All entities in the current room reset to their initial LDtk-defined state — enemies revive, breakables reset, un-picked pickups remain.
- Persistent world state preserved — doors that were permanently opened stay opened, defeated bosses stay defeated, acquired abilities stay acquired.

**Hazard vs. enemy death:** identical code path. Spikes, pits, lava, and enemy-killing-blows all funnel into the same death sequence.

**Combat resources (implied by this model):**
- **Health bar** — depletes from incoming damage; reaching 0 triggers death.
- **FP bar (Focus Points / mana)** — consumed by abilities (TBD which).
- Both refill at benches and on respawn.

**Save points are designer-placed benches**, not implicit room-entry checkpoints. The location of benches is therefore a level-design lever for difficulty pacing — long stretches between benches feel risky.

## Decisions log

| # | Decision | Choice |
|---|---|---|
| 1 | Engine | Bevy (Rust-native, learning Rust gamedev is the goal) |
| 2 | Bevy version policy | Latest stable, accept churn |
| 3 | Physics approach | `avian2d` kinematic — collision queries only, hand-written movement |
| 4 | Level editor | LDtk + `bevy_ecs_ldtk` |
| 5 | Resolution / tile size / art | 384×216 / 16×16 / free asset packs |
| 6 | Jump feel tier | Modern baseline (variable height + coyote + buffer + asymmetric gravity) |
| 7 | Horizontal feel | Hybrid: ground-tight, air-momentum, full air control |
| 8 | Timestep | Fixed 60Hz, render-time interpolation |
| 9 | Wall mechanics | Slide + jump from the start, 6-frame input lockout |
| 10 | Camera | Room-locked, pixel-perfect, hard snap |
| 11 | Project structure | Single crate, plugin-per-feature, defer workspace |
| 12 | Input | `leafwing-input-manager`, keyboard + gamepad from day one |
| 13 | Death / respawn | Respawn at last bench. Lose XP + currency; keep items, resources, abilities. HP + FP refill. No shade. Health bar + FP bar combat resources. |

## Open questions (not yet decided)

- Specific Bevy version pin and current state of `avian2d` / `bevy_ecs_ldtk` ecosystem (verify at `Cargo.toml` time).
- Specific Kenney / itch.io tileset to start with.
- Exact ratio of air accel to ground accel (starting at ~50%, tune against gameplay).
- **Combat mechanics** — melee vs. ranged, attack direction, hitboxes/hurtboxes, attack cancels.
- **FP (Focus Points) generation** — hit-to-gain (Hollow Knight) vs. regen-over-time vs. item-only.
- **Mid-combat healing** — spend FP to heal (Hollow Knight Focus), pickup-based, regen, or refill-at-bench-only.
- **XP purpose** — levels stats automatically, spent at altars/vendors, both, neither (XP is just a score).
- **Currency purpose and acquisition** — drops from enemies, found in chests, both.
- **Knockback** — does taking damage push the player? How long is i-frame invulnerability after a hit?
- Room transition mechanics — how the player moves between LDtk levels (door entities? edge-touch? loaded zones?).
