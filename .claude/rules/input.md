# Input

`leafwing-input-manager` (LWIM) for action-based input. Keyboard + gamepad supported from day one.

## Rules

- Game systems read **named actions** (`PlayerAction::Jump`), never raw `KeyCode` or `GamepadButton`. Bindings live in a single `InputMap` attached to the player entity.
- LWIM exposes `pressed` (level — is it down now?), `just_pressed` (rising edge — went down this frame), and `just_released` (falling edge — went up this frame) per action. The full triple is required:
  - `pressed` for continuous effects (wall slide, hold-to-move).
  - `just_pressed` for discrete impulses (start jump, swing attack).
  - `just_released` for terminating events (cut jump short for variable height).
- The `Move` action is an **axis** (`f32` in `[-1.0, +1.0]`) so analog stick magnitude carries through to movement systems — enables proportional walking from a half-tilted stick without per-system axis blending.

## Starting action set & default bindings

| Action | Type | Keyboard | Gamepad |
|---|---|---|---|
| `Move` | axis | A/D, ←/→ | Left stick X |
| `Jump` | button | Space | South (A / X) |
| `Attack` | button | (TBD) | (TBD) |
| `Dash` | button | (TBD) | (TBD) |
| `Interact` | button | (TBD) | (TBD) |
| `Pause` | button | Escape | Start |

`Attack` / `Dash` / `Interact` bindings are placeholders — set them when those mechanics get implemented.

## Risk mitigation

LWIM occasionally lags Bevy releases. At `Cargo.toml` time, verify there is a published LWIM version compatible with the Bevy version we pin. If not, pin Bevy one minor version back rather than fall back to raw input — the ergonomic delta is large enough that staying current on LWIM is worth a brief Bevy delay.
