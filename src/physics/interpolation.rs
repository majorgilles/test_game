use bevy::math::Vec2;

pub fn lerp_position(previous: Vec2, current: Vec2, alpha: f32) -> Vec2 {
    previous + (current - previous) * alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_position_alpha_zero_returns_previous() {
        // Given a previous and current position with a known gap,
        let previous = Vec2::new(10.0, 20.0);
        let current = Vec2::new(30.0, 50.0);

        // When we sample at alpha = 0.0 (start of the fixed tick),
        let result = lerp_position(previous, current, 0.0);

        // Then we get the previous position back exactly.
        assert_eq!(result, previous);
    }
}
