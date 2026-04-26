# Stack & project structure

## Foundational stack

- **Language:** Rust.
- **Engine:** Bevy, latest stable. Accept the per-release API churn as part of learning. If a new Bevy version drops in the same week as starting work, wait 2–3 weeks for the third-party plugin ecosystem to catch up before upgrading.
- **Physics:** `avian2d` in **kinematic / collision-query mode only**. Movement math (velocity, gravity, jump arc) is hand-written. `avian2d` is used for `Collider` shapes, raycasts, and shape-casts to answer "what did I hit." We do *not* use its rigid-body dynamics — platformer feel needs hand-tuned movement, not realistic physics.
- **Levels:** **LDtk** (ldtk.io), loaded via `bevy_ecs_ldtk`. Chosen over Tiled because LDtk's data model — many "levels" (rooms) arranged spatially in a "world," typed entity instances with strongly-typed fields — matches metroidvania structure directly and avoids writing a custom world-graph layer.

## Project structure

- **Single crate.** `Cargo.toml` at the project root, all code under `src/`. Defer workspace splitting until compile times measurably hurt; Bevy compile time is dominated by monomorphization of generic ECS code, not by crate count, so splitting is rarely the right lever early.
- **Compile-time mitigations** (apply when needed, not preemptively): `bevy/dynamic_linking` feature in dev profile, `lld` linker on Windows, `cargo-watch` for tight loops.
- **Plugin-per-feature.** Each subdirectory defines a Bevy `Plugin`. `main.rs` is short — it composes plugins, nothing else.

**Module layout:**

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
  input.rs          Input mapping (leafwing-input-manager)
  debug/
    mod.rs          Debug overlays, F1 toggle
```

Folder vs. flat-file boundaries are by feature, not technical role — no `components/`, `systems/`, `resources/` directories.
