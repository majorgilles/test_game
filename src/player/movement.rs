use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct MovementConfig {
    pub max_speed: f32,
    pub ground_accel_frames: f32,
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
    fn accel_one_tick_from_rest_with_full_right_input() {
        // Given a player at rest with default movement config,
        let config = MovementConfig::default();

        // When we apply full-right input (direction = +1.0) for one tick,
        let result = next_velocity(0.0, 1.0, &config);

        // Then velocity equals the per-tick accel delta (max_speed / accel_frames).
        let expected = config.max_speed / config.ground_accel_frames;
        assert_eq!(result, expected);
    }

    #[test]
    fn accel_does_not_exceed_max_speed_when_already_at_max() {
        // Given a player already at max_speed,
        let config = MovementConfig::default();
        let starting_velocity = config.max_speed;

        // When we apply full-right input for one tick,
        let result = next_velocity(starting_velocity, 1.0, &config);

        // Then velocity stays at max_speed (clamped).
        assert!(approx_eq(result, config.max_speed));
    }

    #[test]
    fn accel_does_not_exceed_negative_max_speed_when_already_at_negative_max() {
        // Given a player already at -max_speed (full left at top speed),
        let config = MovementConfig::default();
        let starting_velocity = -config.max_speed;

        // When we apply full-left input for one tick,
        let result = next_velocity(starting_velocity, -1.0, &config);

        // Then velocity stays at -max_speed (clamped on the negative side).
        assert!(approx_eq(result, -config.max_speed));
    }

    #[test]
    fn decel_one_tick_when_input_is_released_at_max_speed() {
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
    fn decel_does_not_overshoot_zero_from_a_small_positive_velocity() {
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
    fn half_stick_settles_at_half_max_speed_rather_than_full_max() {
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
    fn easing_stick_from_full_to_half_decelerates_to_half_max() {
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
