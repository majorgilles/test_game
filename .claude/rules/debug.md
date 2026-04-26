# Debug tooling

Debug overlays are first-class, default-built-in, F1/F12 toggled, **shipped in release builds**.

- **`bevy-inspector-egui`** — F12 toggles a runtime entity inspector for any entity, any component.
- **`avian2d` `PhysicsDebugPlugin`** — F1 toggles the built-in collider/raycast renderer in world space.
- **Custom `DebugOverlayPlugin`** (also F1):
  - On-screen text near the player: `grounded`, `against_wall_l`, `against_wall_r`, `coyote_timer`, `jump_buffer`, `velocity (x, y)`.
  - Gizmo arrow from player position along `velocity` vector, length scaled to magnitude.
- All debug code lives under `src/debug/`. Default-on for now; can be moved behind a `cfg(feature = "debug")` later if release-strip becomes a goal.

## Deferred

Frame-stepping, replay recording, and a runtime command console are **deferred** — build them only when a specific bug demands them.
