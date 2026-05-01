use bevy::ecs::resource::Resource;

/// Tunable jump parameters. A resource (not a magic literal) so 5b–5e have
/// something to play against when variable height, coyote, buffer, and
/// asymmetric gravity land on top.
#[derive(Resource)]
pub struct JumpConfig {
    /// Vertical kick applied on a grounded jump press, in pixels/second.
    /// Positive = up; the sign matters because gravity uses negative-y-down.
    pub impulse: f32,
}

impl Default for JumpConfig {
    fn default() -> Self {
        Self { impulse: 260.0 }
    }
}

pub fn apply_jump_impulse(
    current_vy: f32,
    jump_pressed: bool,
    grounded: bool,
    config: &JumpConfig,
) -> f32 {
    // prevents double jumps
    if jump_pressed && grounded {
        return config.impulse;
    }
    current_vy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_jump_impulse_grounded_press_returns_config_impulse() {
        // Given
        let current_vy = -100.0;
        let grounded = true;
        let jump_pressed = true;
        let config = JumpConfig::default();

        // When
        let result = apply_jump_impulse(current_vy, jump_pressed, grounded, &config);

        // Then
        assert_eq!(result, config.impulse);
    }

    #[test]
    fn apply_jump_impulse_airborne_press_returns_current_vy_unchanged() {
        // Given
        let current_vy = -100.0;
        let grounded = false;
        let jump_pressed = true;
        let config = JumpConfig::default();

        // When
        let result = apply_jump_impulse(current_vy, jump_pressed, grounded, &config);

        // Then
        assert_eq!(result, current_vy);
    }

    #[test]
    fn apply_jump_impulse_grounded_not_pressed_returns_current_vy_unchanged() {
        // Given
        let current_vy = -100.0;
        let grounded = true;
        let jump_pressed = false;
        let config = JumpConfig::default();

        // When
        let result = apply_jump_impulse(current_vy, jump_pressed, grounded, &config);

        // Then
        assert_eq!(result, current_vy);
    }
}
