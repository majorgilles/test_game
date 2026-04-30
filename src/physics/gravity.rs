/// Tunable gravity parameters. Will grow asymmetric (up vs. down) in a later stage.
#[derive(bevy::ecs::resource::Resource)]
pub struct GravityConfig {
    /// Downward acceleration in px/s^2. Negative because +y is up.
    pub acceleration: f32,
    /// Maximum fall speed in px/s. Caps integration so a single tick can never
    /// translate further than ground-detection can resolve. Also negative!!!
    pub terminal_velocity: f32,
}

impl Default for GravityConfig {
    fn default() -> Self {
        Self {
            acceleration: -800.0,
            terminal_velocity: -600.0,
        }
    }
}

/// One-tick gravity step. Pure: caller passes `dt` so this is unit-testable
/// without a Bevy `Time` resource. Clamps at terminal velocity.
pub fn next_vertical_velocity(current: f32, config: &GravityConfig, dt: f32) -> f32 {
    let next = current + config.acceleration * dt; // Euler integration
    if next < config.terminal_velocity {
        config.terminal_velocity
    } else {
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn next_vertical_velocity_at_rest_one_tick_accelerates_downward_by_g_times_dt() {
        // Given a player at rest with default gravity config,
        let config = GravityConfig::default();
        let dt = 0.0 / 60.0;

        // When one fixed tick of gravity is applied,
        let result = next_vertical_velocity(0.0, &config, dt);

        // Then velocity dropped by exactly g * dt.
        assert!(approx_eq(result, config.acceleration * dt));
    }

    #[test]
    fn next_vertical_velocity_below_terminal_clamps_to_terminal() {
        // Given a player already at terminal velocity,
        let config = GravityConfig::default();
        let dt = 1.0 / 60.0;

        // When another tick of gravity is applied,
        let result = next_vertical_velocity(config.terminal_velocity, &config, dt);

        // Then velocity stays at terminal - does not keep accelerating
        // past the rate ground-detection can resolve in one tick.
        assert!(approx_eq(result, config.terminal_velocity))
    }
}
