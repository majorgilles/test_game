use bevy::ecs::resource::Resource;

/// Tunable feel parameters for horizontal movement. Lives as a Bevy `Resource`
/// so designers can hot-edit values via the inspector (F12) without recompiling.
///
/// Defaults come from `.claude/rules/platforming-feel.md` and are starting
/// points — they will be tuned against actual level geometry.
#[derive(Resource)]
pub struct MovementConfig {
    /// Top horizontal speed in pixels/second. At 16-px tiles, 112 px/s ≈ 7
    /// tiles/s — the middle of the 6–8 tiles/s target range from the rules.
    pub max_speed: f32,
    /// Number of fixed (60 Hz) ticks to reach `max_speed` from rest with
    /// full-stick input. Fixed-tick units, *not* seconds — the schedule
    /// discipline in `simulation.md` requires frame-counted feel parameters.
    pub ground_accel_frames: f32,
    /// Number of fixed ticks to come to rest from `max_speed` when the stick
    /// is released. Same unit caveat as `ground_accel_frames`.
    pub ground_decel_frames: f32,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            max_speed: 112.0,
            ground_accel_frames: 4.0,
            ground_decel_frames: 5.0,
        }
    }
}

/// Sanitizes a leafwing axis read into the contract `next_velocity` assumes:
/// a real number in `[-1.0, +1.0]`.
///
/// This is the single boundary between leafwing's input pipeline (which we
/// don't fully control — gamepad drivers and HID layers cross OS APIs) and
/// the movement math. Every consumer of `PlayerAction::Move` should funnel
/// the raw read through here so downstream code can rely on a clean value.
///
/// - Out-of-range values would silently let the player exceed `max_speed`
///   downstream (`target = direction * max_speed`), so we clamp at the boundary.
/// - `NaN` propagates through every comparison in `next_velocity` as `false`,
///   producing a silent wrong-direction drift rather than a loud failure —
///   we map it to `0.0` so the player simply stops on malformed input.
pub fn sanitize_axis(raw: f32) -> f32 {
    if raw.is_nan() {
        return 0.0;
    }
    raw.clamp(-1.0, 1.0)
}

/// Advances horizontal velocity by one fixed tick toward the throttle target
/// `direction * max_speed` (per the analog-throttle model in `platforming-feel.md`).
///
/// Caller contract: `direction` must already be sanitized — clean of NaN and
/// inside `[-1.0, +1.0]`. Run the raw axis read through `sanitize_axis` first.
///
/// Two rates govern the approach to target: the accel rate applies when the
/// player is being asked to go *faster* in the input direction (including
/// direction reversal), the decel rate applies when they're being asked to
/// slow down (released stick or reduced stick magnitude). The function never
/// overshoots target — small remaining gaps snap to it.
pub fn next_velocity(current: f32, direction: f32, config: &MovementConfig) -> f32 {
    // Stick magnitude is a throttle: target speed = direction * max_speed.
    // We step `current` toward `target` each tick and never overshoot.
    let target = direction * config.max_speed;

    // Accel rate when we're being asked to go faster *in the input direction*,
    // including direction reversal (signs differ). Decel rate when we're being
    // asked to slow down — released stick (target=0) or reduced stick magnitude.
    let same_direction = current.signum() == target.signum() || current == 0.0;
    let asking_for_more_speed = current.abs() < target.abs();
    let step = if !same_direction || asking_for_more_speed {
        config.max_speed / config.ground_accel_frames
    } else {
        config.max_speed / config.ground_decel_frames
    };

    // Move toward target by `step`, clamped so we don't overshoot.
    if (target - current).abs() <= step {
        target
    } else if target > current {
        current + step
    } else {
        current - step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn sanitize_axis_value_in_range_passes_through_unchanged() {
        // Given a raw axis value already inside [-1.0, +1.0],
        let raw = 0.5;

        // When we sanitize it,
        let result = sanitize_axis(raw);

        // Then it is returned unchanged.
        assert_eq!(result, 0.5);
    }

    #[test]
    fn sanitize_axis_above_one_clamps_to_one() {
        // Given a raw axis value above the +1.0 limit (e.g. hardware/driver glitch),
        let raw = 1.5;

        // When we sanitize it,
        let result = sanitize_axis(raw);

        // Then it is clamped down to +1.0 so downstream code never sees overshoot.
        assert_eq!(result, 1.0);
    }

    #[test]
    fn sanitize_axis_nan_input_returns_zero() {
        // Given a NaN axis read (gamepad driver / HID-layer malformed input —
        // the kind of thing that crosses an external boundary we don't control),
        let raw = f32::NAN;

        // When we sanitize it,
        let result = sanitize_axis(raw);

        // Then it becomes 0.0 — every comparison in next_velocity returns false
        // against NaN, which would otherwise produce a silent wrong-direction drift.
        assert_eq!(result, 0.0);
    }

    #[test]
    fn next_velocity_at_rest_full_right_input_accelerates_by_one_tick_delta() {
        // Given a player at rest with default movement config,
        let config = MovementConfig::default();

        // When we apply full-right input (direction = +1.0) for one tick,
        let result = next_velocity(0.0, 1.0, &config);

        // Then velocity equals the per-tick accel delta (max_speed / accel_frames).
        let expected = config.max_speed / config.ground_accel_frames;
        assert_eq!(result, expected);
    }

    #[test]
    fn next_velocity_at_max_speed_full_right_input_stays_at_max_speed() {
        // Given a player already at max_speed,
        let config = MovementConfig::default();
        let starting_velocity = config.max_speed;

        // When we apply full-right input for one tick,
        let result = next_velocity(starting_velocity, 1.0, &config);

        // Then velocity stays at max_speed (clamped).
        assert!(approx_eq(result, config.max_speed));
    }

    #[test]
    fn next_velocity_at_negative_max_speed_full_left_input_stays_at_negative_max() {
        // Given a player already at -max_speed (full left at top speed),
        let config = MovementConfig::default();
        let starting_velocity = -config.max_speed;

        // When we apply full-left input for one tick,
        let result = next_velocity(starting_velocity, -1.0, &config);

        // Then velocity stays at -max_speed (clamped on the negative side).
        assert!(approx_eq(result, -config.max_speed));
    }

    #[test]
    fn next_velocity_at_max_speed_stick_released_decelerates_by_one_tick_delta() {
        // Given a player at max_speed with no input (stick released),
        let config = MovementConfig::default();
        let starting_velocity = config.max_speed;

        // When one tick passes,
        let result = next_velocity(starting_velocity, 0.0, &config);

        // Then velocity drops by max_speed / decel_frames (the per-tick decel delta).
        let expected = config.max_speed - (config.max_speed / config.ground_decel_frames);
        assert!(approx_eq(result, expected));
    }

    #[test]
    fn next_velocity_small_positive_velocity_stick_released_lands_exactly_at_zero() {
        // Given a player with a small positive velocity smaller than one decel tick,
        let config = MovementConfig::default();
        let decel_per_tick = config.max_speed / config.ground_decel_frames;
        let starting_velocity = decel_per_tick * 0.5; // half a tick from zero

        // When one tick passes with no input,
        let result = next_velocity(starting_velocity, 0.0, &config);

        // Then velocity lands exactly at zero — it does not cross into negative territory.
        assert_eq!(result, 0.0);
    }

    #[test]
    fn next_velocity_held_half_stick_from_rest_settles_at_half_max_speed() {
        // Given a player at rest with default config and a half-tilted stick,
        let config = MovementConfig::default();
        let direction = 0.5;

        // When enough ticks pass for any reasonable accel to complete,
        let mut velocity = 0.0;
        for _ in 0..100 {
            velocity = next_velocity(velocity, direction, &config);
        }

        // Then velocity has settled at half max_speed (the throttle target),
        // not full max_speed — the stick magnitude commands a target speed.
        assert!(approx_eq(velocity, config.max_speed * 0.5));
    }

    #[test]
    fn next_velocity_at_max_speed_eased_to_half_stick_decelerates_to_half_max_speed() {
        // Given a player at full speed (was holding full-right stick),
        let config = MovementConfig::default();
        let starting_velocity = config.max_speed;

        // When the player eases the stick to half-right and holds for many ticks,
        let mut velocity = starting_velocity;
        for _ in 0..100 {
            velocity = next_velocity(velocity, 0.5, &config);
        }

        // Then velocity decelerates from max down to half-max — the new throttle target.
        assert!(approx_eq(velocity, config.max_speed * 0.5));
    }
}
