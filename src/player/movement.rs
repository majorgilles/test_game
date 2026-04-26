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
    if direction == 0.0 {
        // Decay magnitude toward zero, floor at zero, re-apply original sign.
        // Floor prevents overshoot when |current| < decel_per_tick — otherwise
        // velocity would oscillate by ping-ponging across zero each tick.
        let decel_per_tick = config.max_speed / config.ground_decel_frames;
        let new_magnitude = (current.abs() - decel_per_tick).max(0.0);
        return new_magnitude * current.signum();
    }
    let next = current + direction * (config.max_speed / config.ground_accel_frames);
    next.clamp(-config.max_speed, config.max_speed)
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
}
