/// Visual feedback for combat actions
///
/// Handles camera shake, blood particles, damage numbers, and other visual effects.
use bevy::prelude::*;

/// Component for camera shake effect
#[derive(Component, Debug)]
pub struct CameraShake {
    /// Intensity of shake (displacement magnitude)
    pub intensity: f32,

    /// Duration remaining in seconds
    pub duration: f32,

    /// Frequency of shake oscillation
    pub frequency: f32,

    /// Base camera position when shake started
    pub base_position: Vec3,
}

impl CameraShake {
    /// Create a new camera shake effect
    pub fn new(intensity: f32, duration: f32, base_position: Vec3) -> Self {
        Self {
            intensity,
            duration,
            frequency: 20.0, // Default shake frequency
            base_position,
        }
    }

    /// Create shake for a hit effect (base_position will be set when inserted)
    pub fn hit_shake() -> Self {
        Self::new(0.1, 0.15, Vec3::ZERO)
    }

    /// Create shake for a critical hit (base_position will be set when inserted)
    pub fn critical_shake() -> Self {
        Self::new(0.2, 0.25, Vec3::ZERO)
    }
}

/// System to update camera shake
pub fn update_camera_shake(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CameraShake)>,
    mut commands: Commands,
) {
    let mut to_remove = Vec::new();

    for (entity, mut transform, mut shake) in query.iter_mut() {
        // Store base position on first frame (when base_position is zero)
        if shake.base_position == Vec3::ZERO {
            shake.base_position = transform.translation;
        }

        if shake.duration <= 0.0 {
            // Reset to base position before removing
            transform.translation = shake.base_position;
            to_remove.push(entity);
            continue;
        }

        // Calculate shake offset using sine wave
        let elapsed = time.elapsed_secs();
        let shake_x = (elapsed * shake.frequency).sin() * shake.intensity;
        let shake_y = (elapsed * shake.frequency * 1.3).cos() * shake.intensity * 0.7;
        let shake_z = (elapsed * shake.frequency * 0.8).sin() * shake.intensity * 0.5;

        // Set position to base + shake offset
        transform.translation = shake.base_position + Vec3::new(shake_x, shake_y, shake_z);

        // Decrease duration
        shake.duration -= time.delta_secs();
    }

    // Clean up expired shakes
    for entity in to_remove {
        commands.entity(entity).remove::<CameraShake>();
    }
}

/// Component for damage number floating text
#[derive(Component, Debug)]
pub struct DamageNumber {
    /// Time remaining before despawn
    pub lifetime: f32,

    /// Initial spawn time for animation
    pub spawn_time: f32,
}

impl DamageNumber {
    pub fn new() -> Self {
        Self {
            lifetime: 1.5,
            spawn_time: 1.5,
        }
    }
}

/// System to update damage number positions and lifetime
pub fn update_damage_numbers(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut DamageNumber, &mut TextColor)>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut damage_num, mut text_color) in query.iter_mut() {
        // Move upward
        transform.translation.z += time.delta_secs() * 2.0;

        // Fade out based on remaining lifetime
        let alpha = (damage_num.lifetime / damage_num.spawn_time).clamp(0.0, 1.0);
        text_color.0 = text_color.0.with_alpha(alpha);

        // Update lifetime
        damage_num.lifetime -= time.delta_secs();

        // Despawn when expired
        if damage_num.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawn a damage number at the given world position
pub fn spawn_damage_number(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    position: Vec3,
    damage: i32,
    critical: bool,
) {
    let color = if critical {
        Color::srgb(1.0, 0.8, 0.2) // Gold for crits
    } else {
        Color::srgb(1.0, 0.2, 0.2) // Red for normal hits
    };

    let font_size = if critical { 48.0 } else { 32.0 };

    commands.spawn((
        Text2d::new(damage.to_string()),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        Transform::from_translation(position + Vec3::new(0.0, 0.0, 2.0)),
        DamageNumber::new(),
    ));
}

/// Component for blood particle effect
#[derive(Component, Debug)]
pub struct BloodParticle {
    /// Velocity of particle
    pub velocity: Vec3,

    /// Time remaining before despawn
    pub lifetime: f32,
}

impl BloodParticle {
    pub fn new(velocity: Vec3) -> Self {
        Self {
            velocity,
            lifetime: 0.5,
        }
    }
}

/// System to update blood particles
pub fn update_blood_particles(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut BloodParticle)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, mut particle) in query.iter_mut() {
        // Apply velocity
        transform.translation += particle.velocity * dt;

        // Apply gravity (Z- since Z+ is up)
        particle.velocity.z -= 9.8 * dt;

        // Update lifetime
        particle.lifetime -= dt;

        // Despawn when expired
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawn blood particles at the given position
pub fn spawn_blood_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    count: u32,
) {
    let blood_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.0, 0.0),
        unlit: true,
        ..default()
    });

    let particle_mesh = meshes.add(Sphere::new(0.05));

    for _ in 0..count {
        // Random velocity
        let velocity = Vec3::new(
            (rand::random::<f32>() - 0.5) * 4.0,
            (rand::random::<f32>() - 0.5) * 4.0,
            rand::random::<f32>() * 3.0,
        );

        commands.spawn((
            Mesh3d(particle_mesh.clone()),
            MeshMaterial3d(blood_material.clone()),
            Transform::from_translation(position),
            BloodParticle::new(velocity),
        ));
    }
}
