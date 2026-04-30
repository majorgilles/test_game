use bevy::ecs::query::With;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::{Query, Res};
use bevy::time::{Fixed, Time};
use leafwing_input_manager::action_state::ActionState;

use crate::input::PlayerAction;
use crate::physics::kinematics::{Position, Velocity};
use crate::player::Player;

/// Tunable feel parameters for horizontal movement, kept as a `Resource` for
/// inspector hot-edits.
#[derive(Resource)]
pub struct MovementConfig {
    /// Top horizontal speed in pixels/second.
    pub max_speed: f32,
    /// Fixed-tick (60 Hz) units, not seconds — feel is tuned in frames.
    pub ground_accel_frames: f32,
    /// Fixed-tick units. See `ground_accel_frames`.
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

/// Clamps a raw axis read into `[-1.0, +1.0]` and maps `NaN` to `0.0` so
/// `next_velocity` never sees out-of-contract input.
pub fn sanitize_axis(raw: f32) -> f32 {
    if raw.is_nan() {
        return 0.0;
    }
    raw.clamp(-1.0, 1.0)
}

/// One-tick step of horizontal velocity toward `direction * max_speed`.
/// Caller must pre-sanitize `direction`; reversal uses accel rate, slowdown
/// uses decel rate, never overshoots target.
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

/// FixedUpdate system: reads `Move`, steps velocity, integrates position.
pub fn apply_horizontal_movement(
    time: Res<Time<Fixed>>,
    config: Res<MovementConfig>,
    mut query: Query<(&ActionState<PlayerAction>, &mut Velocity, &mut Position), With<Player>>,
) {
    for (actions, mut velocity, mut position) in &mut query {
        let direction = sanitize_axis(actions.clamped_value(&PlayerAction::Move));
        velocity.0.x = next_velocity(velocity.0.x, direction, &config);
        position.0.x += velocity.0.x * time.delta_secs();
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
    fn next_velocity_at_max_speed_full_left_input_decelerates_by_accel_rate() {
        // Given a player at full positive max speed (was holding full-right),
        let config = MovementConfig::default();
        let starting_velocity = config.max_speed;

        // When the player slams the stick fully in the opposite direction
        // (full-left, direction = -1.0) for one tick,
        let result = next_velocity(starting_velocity, -1.0, &config);

        // Then velocity drops by the *accel* per-tick delta — direction reversal
        // is "asking to go faster the other way", an accel concern. Using the
        // decel rate would make turn-arounds feel sluggish.
        let accel_per_tick = config.max_speed / config.ground_accel_frames;
        let expected = starting_velocity - accel_per_tick;
        assert!(approx_eq(result, expected));
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
