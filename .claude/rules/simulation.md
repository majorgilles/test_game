# Simulation model

This rule covers **when** game logic runs — the schedule discipline. It is one of the most important rules in the project because getting it wrong produces bugs that are subtle, framerate-dependent, and only show up on someone else's hardware.

## The problem this solves

Monitors run at different refresh rates: 60 Hz on a basic laptop, 144 Hz on a gaming monitor, 240 Hz on a high-end one. By default, a game's update loop runs once per monitor refresh — so on a 240 Hz machine, game logic ticks 4× as often as on a 60 Hz machine.

For most code this doesn't matter as long as you multiply by `delta_time`. But platformer feel is tuned in **frames**, not seconds:

- "Coyote time: ~6 frames after leaving a ledge."
- "Jump buffer: ~6 frames before landing."
- "Ground accel: reach max speed in ~4 frames."

If a "frame" means 1/60s on one machine and 1/240s on another, **the same code is a different game on different hardware**. The 240 Hz player's coyote window is 25ms instead of 100ms — a 4× harder game, by accident. This is a real shipped bug class (Dark Souls animations, Devil May Cry 5 stamina, Skyrim physics).

## The solution: separate physics from rendering

Bevy splits its update loop into two scheduling buckets:

- **`FixedUpdate`** runs at a **fixed rate** (we configure: **60 Hz**). It runs the same number of times per second regardless of monitor refresh. If the renderer is slow this frame, FixedUpdate will run *twice* next frame to catch up. If the renderer is fast, FixedUpdate will be skipped that frame and run on the next one. The point: across any wall-clock second, FixedUpdate ticks exactly ~60 times.
- **`Update`** runs at **monitor rate** (60, 144, 240 Hz, whatever the screen refreshes at). This is where rendering and pure-visual logic live.

**Anything that affects the simulation goes in `FixedUpdate`.** Movement integration, collision resolution, gravity, jump physics, coyote-time and jump-buffer countdowns, animation **state derivation** (deciding *which* animation to play), input edge-detection consumption.

**Anything purely visual goes in `Update`.** Render-time interpolation (see below), animation **frame advancement** (which frame of the running animation to draw), debug overlay rendering.

## The new problem: smooth motion on high-refresh monitors

If physics ticks at 60 Hz and the monitor renders at 144 Hz, **the screen redraws ~2.4× between each physics tick.** Without intervention, the renderer sees the same physics state for two consecutive redraws, then suddenly the next physics tick lands and position jumps. The result: the sprite moves in visible 60 Hz steps even though the screen refreshes at 144 Hz. Looks stuttery — the bug Bevy users call "fixed-step jitter."

## The fix: render-time interpolation

Each physics tick, save the *previous* position alongside the *new* one:

```rust
#[derive(Component)]
struct PreviousPosition(Vec2);
```

When rendering, ask Bevy how far through the *current* physics tick we are — `Time<Fixed>::overstep_fraction()` returns a value in `0.0..=1.0`:

- `0.0` → we're right at the start of the tick; render at `PreviousPosition`.
- `0.5` → halfway through; render at the midpoint.
- `1.0` → at the end of the tick; render at the new position.

The interpolation system runs in `Update` and writes to the sprite's `Transform`. The sprite slides smoothly between physics states, even though the simulation only ticks 60 times per second.

This means **the player's "real" position is *not* their `Transform`** during gameplay. The `Transform` is the *visual*, lerped from physics state. The simulation reads/writes its own position component (e.g. `Position(Vec2)`); only the interpolation system writes the `Transform`. Conflating the two breaks interpolation.

## Animation lives in both schedules

Animation has two distinct concerns:

1. **State derivation** — *which* animation should be playing? `Idle`, `Run`, `JumpRise`, `WallSlide`? This depends on physics state (grounded? velocity?), so it runs in `FixedUpdate`. State must agree with the 60 Hz tick.
2. **Frame advancement** — *which frame* of the current animation should be drawn right now? This is purely visual and must be smooth at any refresh rate, so it runs in `Update`. Tying frame advancement to FixedUpdate would lock animation to 60 fps even on a 240 Hz monitor.

## Schedule discipline (the rule)

| Schedule | Runs at | Belongs here |
|---|---|---|
| `FixedUpdate` | exactly ~60 Hz | physics, movement, gravity, jump/coyote/buffer timers, animation **state** derivation, anything that mutates simulation state |
| `Update` | monitor rate (60/144/240+) | render-time interpolation, animation **frame** advancement, debug overlay rendering, anything purely visual |

If you find yourself writing a system that "needs to read physics state but also needs to be smooth on high-refresh monitors," that's a sign you're conflating the two concerns — split it: derive state in `FixedUpdate`, advance the visual in `Update`.

## Why the choice is non-negotiable for this project

- Coyote time and jump buffer (decision 6) are defined in fixed-tick units. They cease to mean anything if "tick" is variable.
- Variable jump cutoff (decision 6) cuts upward velocity on the release frame. If "frame" is variable, the cutoff lands at different velocities on different hardware → different jump heights.
- Determinism across hardware enables reasoning about feel and (eventually) replays / netplay if we ever go there.

The cost is ~30 lines for the interpolation system and the discipline to put each new system in the right schedule. The benefit is that "the jump feels right on my machine" actually means "the jump feels right on every machine."
