# Testing

## Why we test

Game code has two failure modes that matter for this project:

1. **Math bugs** in feel-tuning code (jump arc, accel curves, coyote countdown). These are deterministic, easy to unit-test, and catastrophic to leave to playtest discovery — they corrupt the feel-tuning loop ("does it feel right?" stops being meaningful when the math is wrong).
2. **API translation bugs** in Bevy glue. We hit two of these in stage 2 (the `PluginGroup` import, the private `projection` module). They surface at compile time and don't need test coverage; `cargo check` is the test.

This rule covers (1). The discipline is: **anything that is pure math gets a unit test**. Bevy ECS plumbing — system registration, component queries, plugin composition — is verified by `cargo run` and human play.

## Where tests live

**Inline `#[cfg(test)] mod tests`** at the bottom of the module being tested. Standard Rust convention.

```rust
// src/player/movement.rs

pub fn next_velocity(/* ... */) -> f32 { /* ... */ }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_describing_observable_behavior() {
        // Given ...
        // When  ...
        // Then  ...
    }
}
```

- `#[cfg(test)]` strips the test module from release builds — zero runtime cost.
- `use super::*;` lets tests reach private items in the parent module. Test private functions only when they encode tricky behavior worth pinning; prefer testing the public API.
- Integration tests (cross-module, app-level) go in a top-level `tests/` directory. We don't have any yet; add one only when the test would *exclusively* exercise a public crate API.

## Test naming

**Name tests after the behavior, not the function.** A test name reads as a sentence about what the system does.

- ✅ `accel_one_tick_from_rest_with_full_right_input`
- ✅ `decel_does_not_overshoot_zero`
- ✅ `analog_half_stick_produces_half_accel`
- ❌ `test_next_velocity_1`
- ❌ `next_velocity_works`

If renaming the function under test would force renaming the test, the test name is wrong. The behavior didn't change — the implementation moved.

## Test body structure: Given / When / Then

Use **Given / When / Then** comment blocks inside every test. Adopted from BDD; equivalent to Arrange / Act / Assert. The labels act as a local table of contents — you scan a test and immediately see the setup, the action, and the verification.

```rust
#[test]
fn analog_half_stick_produces_half_accel() {
    // Given a player at rest with default movement config,
    let config = MovementConfig::default();
    let starting_velocity = 0.0;

    // When we apply a half-tilted stick (direction = 0.5) for one tick,
    let result = next_velocity(starting_velocity, 0.5, &config);

    // Then the resulting velocity is half of what a full stick produces.
    let full_tick_delta = config.max_speed / config.ground_accel_frames;
    assert_eq!(result, full_tick_delta * 0.5);
}
```

Rules of thumb:

- **One logical action per test.** Multiple "When" blocks is a smell — split the test.
- **Given is the setup**, including config and starting state. Trivial defaults can live on a single line.
- **When is one call** — the system under test, with the inputs you constructed in Given.
- **Then is one or more assertions** about the result. Multiple asserts are fine if they all describe the same observable behavior (e.g., "velocity clamps and direction is preserved"); prefer a single assert when possible.

## What to test

**Test pure functions.** They are deterministic, fast, and free of ECS concerns. Extract math out of systems into pure helpers (`next_velocity`, `integrate_position_1d`, `apply_jump_cutoff`) and unit-test them. The system itself becomes a thin wrapper: read components → call helper → write components.

**Don't test the wrapper.** Whether a system is registered to `FixedUpdate` or `Update` is not behavior — it is an architectural choice covered by `simulation.md`. If the wiring is wrong, the user notices in seconds. If the math is wrong, the user notices over weeks of subtly bad feel.

**Don't test Bevy.** No tests for "spawning an entity adds it to the world" or "a Query returns matching entities." Bevy's own tests cover that.

**Don't test third-party crates.** No tests that LWIM correctly converts gamepad events to `ActionState`. Trust the dep; if it's broken we replace it.

## When to skip a unit test

Reasonable exceptions, in priority order:

1. **The system has no extractable math** — e.g., a one-line `transform.translation.x = position.0.x` sync system. The test would just re-state the assignment. Skip.
2. **The behavior is purely visual** — animation frame advancement, sprite picking, color tinting. Verify by eye, not by assertion.
3. **The behavior emerges from Bevy's scheduler** — e.g., system ordering inside a schedule. Test by running the app, not by mocking the scheduler.

In every other case, write the test first.

## Running tests

- `cargo test` — runs all tests, prints results.
- `cargo test --lib` — only unit tests in `src/`.
- `cargo test name_substring` — only tests whose names contain the substring.

Tests run in parallel by default. Make tests independent — no shared global state, no filesystem assumptions, no test that depends on running before/after another. If a test must serialize, it's a sign of leaky state, not a reason for `--test-threads=1`.

## Assertion choices

- `assert_eq!(a, b)` — equality. Use for `f32` comparisons only when the math is truly exact (e.g., the result of a single multiplication-and-divide of nice numbers). For accumulated math or divisions that don't divide evenly, use a tolerance:
  ```rust
  fn approx_eq(a: f32, b: f32) -> bool { (a - b).abs() < 1e-5 }
  assert!(approx_eq(result, expected));
  ```
  Don't pull in a float-comparison crate yet — a five-line helper is enough until tests outgrow it.
- `assert!` — boolean conditions.
- `assert_ne!` — inequality. Used rarely; usually `assert!(...)` reads better.

## Sources / further reading

- [Rust Book — How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [Rust Book — Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- [Martin Fowler — Given When Then](https://martinfowler.com/bliki/GivenWhenThen.html)
