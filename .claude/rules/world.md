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

Start simple, add complexity only when a room demands it.

- **Room-locked.** Camera position = current room's center, in world space. One room fills the viewport.
- **Pixel-perfect.** Camera translation is rounded to whole pixels each frame after positioning.
- **Hard snap on transition.** No smooth blend between rooms (yet). Add a blend later only if the snap feels jarring.
- **No deadzone scrolling yet.** When we hit a room genuinely too large for one viewport, add deadzone-based smooth-follow as a *second* camera mode. Until then, room-locked is enough.

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
