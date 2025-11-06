/// Combat system module
/// 
/// Handles weapon attacks, damage calculation, and combat states.
/// Organized into submodules for clarity and maintainability.

pub mod attack_state;
pub mod audio_feedback;
pub mod damage;
pub mod status_effects;
pub mod weapon;
pub mod visual_feedback;

pub use attack_state::{AttackState, CombatInput, StateTransition};
pub use audio_feedback::{CombatAudio, play_swing_sound, play_hit_sound};
pub use damage::{calculate_damage, DamageResult, DamageType};
pub use status_effects::{StatusEffect, StatusEffectType, update_status_effects, apply_status_effect};
pub use weapon::{WeaponDefinition, WeaponDefinitions};
pub use visual_feedback::{
    CameraShake, DamageNumber, BloodParticle,
    update_camera_shake, update_damage_numbers, update_blood_particles,
    spawn_damage_number, spawn_blood_particles,
};
