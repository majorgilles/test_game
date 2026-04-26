# Platforming feel

## Jump — modern baseline

Implementation order matters: each addition should be a felt improvement before moving to the next.

1. **Variable jump height.** Hold to jump higher; release early to cut the jump short. Implemented by setting `velocity.y *= 0.5` (or similar) the frame the jump button releases *if* `velocity.y > 0`.
2. **Coyote time.** ~6 frames of grace after walking off a ledge during which a jump press still counts as a grounded jump. Tracked via a `coyote_timer` that resets on grounded state and counts down while airborne.
3. **Jump buffer.** ~6 frames of grace where a jump press *just before* landing is queued and consumed on landing. Tracked via a `jump_buffer_timer` that is set on jump press and counts down each tick.
4. **Asymmetric gravity.** Lower gravity going up (~1× base), higher going down (~1.5–2× base). Optionally a third, even higher value for "fast fall" if down is held while airborne — to be decided later.

We are **not** implementing apex hangtime, dynamic friction tuning, or jump-corner-correction at this stage. They are correctly tuned only against real level geometry, not in a vacuum.

## Horizontal — hybrid, ground-tight / air-momentum

Starting parameters (will be tuned against actual level geometry once a test room exists):

- Ground accel: reach max speed in ~4 frames (~0.07s)
- Ground decel: stop from max speed in ~5 frames (~0.08s)
- Air accel: reach max speed in ~10 frames (~0.17s) — roughly half ground accel
- Air decel: ~12 frames
- Max speed: ~6–8 tiles/second → ~96–128 px/s at 16-px tiles
- **Full air control** (player can press the opposite direction mid-jump), but the reduced air accel means direction reversal is not instant — jumps feel committed without being locked.

## Wall mechanics

Wall slide + wall jump available **from the start** (not gated behind an upgrade at this prototyping stage).

- **Wall slide.** When airborne, moving into a wall, and `velocity.y < 0`, clamp fall speed to a slow max (~60 px/s).
- **Wall jump.** On jump press while wall-sliding, `velocity = (-wall_normal * kickoff_x, jump_velocity_y)`. Variable jump height applies (same vertical impulse as ground jump, same release-cut behavior).
- **Input lockout.** ~6 frames of horizontal input lockout after kickoff so the player can't immediately re-stick to the same wall.
- Full wall climb (free vertical movement on walls) is **not** available — that's reserved as a potential late-game upgrade.
