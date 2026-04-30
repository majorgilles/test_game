use bevy::input::gamepad::GamepadButton;
use bevy::input::keyboard::KeyCode;
use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::input_processing::WithAxisProcessingPipelineExt;
use leafwing_input_manager::prelude::{GamepadControlAxis, VirtualAxis};

/// Player-facing action vocabulary. Systems read these instead of raw
/// `KeyCode` / `GamepadButton` so bindings live in one place.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    /// Horizontal axis in `[-1.0, +1.0]`. Analog magnitude reaches movement
    /// intact — required for the half-stick-throttle model.
    #[actionlike(Axis)]
    Move,
    /// Jump button. Wired now to prove input plumbing; physics lands later.
    Jump,
}

/// Default keyboard + gamepad bindings per `.claude/rules/input.md`.
pub fn default_input_map() -> InputMap<PlayerAction> {
    InputMap::default()
        .with_axis(PlayerAction::Move, VirtualAxis::ad())
        .with_axis(PlayerAction::Move, VirtualAxis::horizontal_arrow_keys())
        .with_axis(
            PlayerAction::Move,
            GamepadControlAxis::LEFT_X.with_deadzone_symmetric(0.1),
        )
        .with(PlayerAction::Jump, KeyCode::Space)
        .with(PlayerAction::Jump, GamepadButton::South)
}
