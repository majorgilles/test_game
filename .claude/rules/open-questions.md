# Open questions

Decisions deferred until they're forced by code or scope. Resolve when the relevant system is being built.

## Stack & assets

- Specific Bevy version pin and current state of `avian2d` / `bevy_ecs_ldtk` / `leafwing-input-manager` ecosystem (verify at `Cargo.toml` time).
- Specific Kenney / itch.io tileset *and character pack* to start with — frame counts baked into the assets dictate animation timing and locomotion tuning.

## Platforming feel

- Exact ratio of air accel to ground accel (starting at ~50%, tune against gameplay).
- Whether to add a "fast-fall" gravity multiplier when down is held airborne.

## Combat (not yet discussed)

- Melee vs. ranged.
- Attack direction (facing-only, 4-way, 8-way, free aim).
- Hitboxes / hurtboxes representation.
- Attack cancels (can you cancel into dash, jump, another attack?).
- Knockback — does taking damage push the player? How long is i-frame invulnerability after a hit?

## Combat resources

- **FP (Focus Points) generation** — hit-to-gain (Hollow Knight) vs. regen-over-time vs. item-only.
- **Mid-combat healing** — spend FP to heal (Hollow Knight Focus), pickup-based, regen, or refill-at-bench-only.
- **XP purpose** — levels stats automatically, spent at altars/vendors, both, neither (XP is just a score).
- **Currency purpose and acquisition** — drops from enemies, found in chests, both.

## Progression

- Door entities and locked-passage system (deferred from room-transition decision).
- Upgrade / ability acquisition model (Metroid suit pickups vs. Hollow Knight charms vs. Castlevania equip slots).
