# Art, resolution, and rendering

## Resolution

- **Internal canvas:** 384×216 (16:9). Renders to a fixed-size canvas and is upscaled (integer scale) to the window — ×5 to 1080p, ×10 to 4K. No blurry scaling, no letterbox math.
- **Tile size:** 16×16.

## Art source

Free asset packs (Kenney, OpenGameArt, itch.io free packs) while focus is on code. We are not drawing original art at this stage.

A specific tileset and character pack must be picked **before** locomotion tuning — the chosen sprite's run-loop frame count and stride length dictate locomotion speed.

## Animation

Hand-rolled state machine driven by a `match` on physics state. No state-machine crate.

- **State-derivation** runs in `FixedUpdate` (reads physics state, must agree with the 60Hz tick).
- **Frame advancement** runs in `Update` (visual-only, must be smooth on 120/144/240 Hz monitors — tying it to FixedUpdate would lock animation framerate to 60).
- **Asset format:** plain PNG spritesheets via Bevy's built-in `TextureAtlas`. Aseprite path is rejected — free packs are not Aseprite-formatted.
- **Frame counts and timing are baked into the chosen sprite pack** — measure, don't decide. Locomotion speed should be tuned to match the run-loop stride length of the chosen sprite, otherwise the character moonwalks.

**Initial state set:**

| State | Trigger | Loop |
|---|---|---|
| `Idle` | grounded, `\|velocity.x\| < threshold` | yes |
| `Run` | grounded, moving | yes |
| `JumpRise` | airborne, `velocity.y > 0` | yes |
| `JumpFall` | airborne, `velocity.y ≤ 0` | yes |
| `WallSlide` | airborne, against wall, falling | yes |
| `Land` | grounded transition with significant fall speed | one-shot → Idle |

`Land` is optional for the prototype — can ship without it.

## Pixel-perfect rendering

Camera translation is rounded to whole pixels each frame after positioning. Non-negotiable for pixel art — sub-pixel sampling makes sprites jitter against the tilemap.
