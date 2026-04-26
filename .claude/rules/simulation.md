# Simulation model

- **Fixed-timestep physics at 60 Hz.** All movement, collision, gravity, jump-cutoff, coyote-time, and jump-buffer logic runs in Bevy's `FixedUpdate` schedule. "N frames" of coyote/buffer means N fixed steps regardless of monitor refresh rate.
- **Render-time interpolation.** Sprites' `Transform` is lerped between `previous_position` and `current_position` by `fixed_time.overstep_fraction()` so visuals are smooth on 120/144/240Hz monitors despite physics ticking at 60Hz.
- **Why fixed:** determinism across hardware, framerate-independent feel, and timing-sensitive mechanics (variable jump cutoff, coyote time) only mean something when a "frame" is a fixed duration.

## Schedule discipline

- `FixedUpdate` — physics, movement, jump/coyote/buffer timers, animation **state derivation**, anything that reads or mutates simulation state.
- `Update` — animation **frame advancement**, render-time interpolation, debug overlay rendering, anything purely visual.

Animation state-derivation lives in `FixedUpdate` (it reads physics state); frame-advancement lives in `Update` (it must be smooth on high-refresh monitors).
