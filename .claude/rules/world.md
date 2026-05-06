# World: rooms, camera, death/respawn

## Room transitions

**Edge-touch only** for now. No door entities yet — defer until locked-door / ability-gate progression matters.

- Each LDtk level has invisible edge regions (left, right, top, bottom). Player hitbox overlapping an edge region triggers transition to the LDtk-adjacent level in that direction (LDtk's spatial layout answers adjacency).
- Transition sequence (~0.3s total):
  1. Freeze player input.
  2. Fade screen to black (~0.15s).
  3. Despawn old room entities, spawn new room from LDtk.
  4. Reposition player at the mirrored edge of the new room.
  5. **Velocity preserved** across the transition — running entry into the next room feels continuous.
  6. Snap camera to new room center (room-locked, so this is just centering).
  7. Fade screen back from black (~0.15s).
  8. Unfreeze input.
- No slide-pan; fade is dead simple, hides any level-load hitch, reads cleanly.

Doors, locked passages, and fast-travel are **deferred** until upgrade/progression systems are designed.

## Camera

Follow the player; never reveal out-of-room space.

- **Follow.** Camera target = player position each `Update` tick. Centered on the player when the room is large enough; otherwise clamped (see below).
- **Clamp to room bounds.** Camera position is clamped per-axis so the viewport stays inside the current room's rect. Near a wall the player drifts off-center toward that wall — the alternative (revealing out-of-room cells, or letterboxing) is worse. If a room is *smaller* than the viewport on an axis, that axis falls back to room-center.
- **Pixel-perfect.** Camera translation is rounded to whole pixels each frame, applied *after* clamping. Sub-pixel sampling makes sprites jitter against the tilemap.
- **Hard snap on room transition.** No smooth blend between rooms. Add a blend later only if the snap feels jarring.
- **No deadzone, no smoothing/lerp yet.** Camera tracks the player 1:1 each frame. Adding a deadzone (player moves freely inside a small box before camera engages) and follow smoothing are real feel knobs, but they're cheap to add later and only worth tuning against real gameplay.

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
