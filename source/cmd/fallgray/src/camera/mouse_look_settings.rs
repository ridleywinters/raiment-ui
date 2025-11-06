/// Mouse look settings resource
///
/// Controls cursor lock state, mouse smoothing, and rotation parameters
/// for first-person camera controls.
use bevy::prelude::*;

/// Resource for mouse look settings
#[derive(Resource)]
pub struct MouseLookSettings {
    /// Whether the cursor is currently locked for FPS controls
    pub cursor_locked: bool,

    /// Whether to use smooth (velocity-based) mouse movement
    pub smooth_enabled: bool,

    /// Decay factor for smooth mouse (0.0-1.0, lower = more smoothing)
    pub smooth_factor: f32,

    /// Maximum rotation speed in radians per frame to prevent spinning
    pub rotation_limit: f32,

    /// Maximum pitch angle in radians (prevents looking too far up/down)
    pub pitch_limit: f32,

    /// Whether to invert the Y-axis (mouse up = look down)
    pub invert_y: bool,
}

impl Default for MouseLookSettings {
    fn default() -> Self {
        Self {
            cursor_locked: false, // Start unlocked for safety
            smooth_enabled: true,
            smooth_factor: 0.5,
            rotation_limit: 0.35,
            pitch_limit: 70.0_f32.to_radians(), // ±70 degrees
            invert_y: false,
        }
    }
}
