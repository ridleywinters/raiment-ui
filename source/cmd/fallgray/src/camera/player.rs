use bevy::prelude::*;

/// Player/Camera entity marker with movement and rotation speeds
#[derive(Component)]
pub struct Player {
    pub speed: f32,

    /// Accumulators for smooth mouse movement
    pub yaw_velocity: f32,
    pub pitch_velocity: f32,
}
