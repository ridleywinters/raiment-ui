use super::components::*;
use super::mouse_look_settings::MouseLookSettings;
use super::player::Player;
use crate::collision::PLAYER_RADIUS;
use crate::console::ConsoleState;
use crate::map::Map;
use crate::scripting::CVarRegistry;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use rand::Rng;

pub fn update_camera_control_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    map: Res<Map>,
    console_state: Res<ConsoleState>,
    mouse_look: Res<MouseLookSettings>,
    cvars: Res<CVarRegistry>,
    mut query: Query<(&mut Transform, &mut Player)>,
    ui_interaction_query: Query<&Interaction>,
) {
    // Don't process camera controls if console is open
    if console_state.visible {
        return;
    }

    for (mut transform, mut player) in query.iter_mut() {
        let dt = time.delta_secs();

        // Mouse look input - only process if cursor is locked, console is closed, and not hovering UI
        let ui_hovered = ui_interaction_query.iter().any(|i| *i != Interaction::None);
        let can_mouse_look = mouse_look.cursor_locked && !console_state.visible && !ui_hovered;

        if can_mouse_look {
            // Read mouse sensitivity from CVar
            let mouse_sensitivity = cvars.get_f32("mouse.sensitivity");

            // Read invert_y setting from CVar (1 = inverted, 0 = normal)
            let invert_y = cvars.get_i32("mouse.invert_y") != 0;
            let invert_factor = if invert_y { 1.0 } else { -1.0 };

            // Check if smooth mouse is enabled via CVar (1 = enabled, 0 = disabled)
            let smooth_enabled = cvars.get_i32("mouse_smooth") != 0;

            // Accumulate mouse motion
            for event in mouse_motion.read() {
                let yaw_input = -event.delta.x * mouse_sensitivity;
                let pitch_input = -event.delta.y * mouse_sensitivity * invert_factor;

                if smooth_enabled {
                    // Add to velocity accumulators for smooth mode
                    player.yaw_velocity += yaw_input;
                    player.pitch_velocity += pitch_input;
                } else {
                    // Direct mode - apply rotation immediately via arrow key delta variables
                    // (will be processed in the rotation section below)
                }
            }
        } else {
            // Clear mouse motion events when not using mouse look
            mouse_motion.clear();
        }

        // Check if modifier keys are pressed
        let ctrl_pressed =
            input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);

        // Movement input (WASD + RF)
        // WASD moves in the XY plane, RF moves along Z axis
        let mut movement_xy = Vec2::ZERO; // Movement in XY plane
        let mut movement_z = 0.0; // Movement along Z axis

        if !ctrl_pressed {
            if input.pressed(KeyCode::KeyW) {
                movement_xy.y += 1.0;
            }
            if input.pressed(KeyCode::KeyS) {
                movement_xy.y -= 1.0;
            }
            if input.pressed(KeyCode::KeyA) {
                movement_xy.x -= 1.0;
            }
            if input.pressed(KeyCode::KeyD) {
                movement_xy.x += 1.0;
            }
            if input.pressed(KeyCode::KeyF) {
                movement_z -= 1.0;
            }
            if input.pressed(KeyCode::KeyR) {
                movement_z += 1.0;
            }
        }

        // Rotation input (Arrow keys)
        // Arrow left/right rotates around Z axis (yaw)
        // Arrow up/down changes pitch (looking up/down)
        // Read arrow sensitivity from CVar
        let arrow_sensitivity = cvars.get_f32("arrow_sensitivity");
        let mut yaw_delta = 0.0;
        let mut pitch_delta = 0.0;

        if input.pressed(KeyCode::ArrowLeft) {
            yaw_delta += arrow_sensitivity * dt;
        }
        if input.pressed(KeyCode::ArrowRight) {
            yaw_delta -= arrow_sensitivity * dt;
        }
        if input.pressed(KeyCode::ArrowUp) {
            pitch_delta += arrow_sensitivity * dt;
        }
        if input.pressed(KeyCode::ArrowDown) {
            pitch_delta -= arrow_sensitivity * dt;
        }

        // Get current yaw from the forward direction projected onto XY plane
        {
            let scale = if yaw_delta.abs() > 0.0 {
                0.25
            } else if movement_xy.length_squared() > 0.0 {
                0.1
            } else {
                0.0
            };

            let forward_3d = transform.forward().as_vec3();
            let forward_xy = Vec2::new(forward_3d.x, forward_3d.y);
            let yaw = forward_xy.y.atan2(forward_xy.x);

            let snap_increment = std::f32::consts::PI / 4.0;
            let mut yaw_snap = (yaw / snap_increment).round() * snap_increment;

            if yaw_delta < 0.0 && yaw_snap > yaw {
                yaw_snap -= snap_increment;
            } else if yaw_delta > 0.0 && yaw_snap < yaw {
                yaw_snap += snap_increment;
            }

            let max = scale * arrow_sensitivity * dt;
            yaw_delta += (yaw_snap - yaw).clamp(-max, max);
        }

        // Apply smooth mouse rotation (velocity-based)
        let smooth_enabled = cvars.get_i32("mouse_smooth") != 0;
        if smooth_enabled {
            let dt_factor = dt * 60.0; // Frame-rate independence (60 FPS baseline)

            // Clamp velocities to rotation limit
            player.yaw_velocity = player
                .yaw_velocity
                .clamp(-mouse_look.rotation_limit, mouse_look.rotation_limit);
            player.pitch_velocity = player
                .pitch_velocity
                .clamp(-mouse_look.rotation_limit, mouse_look.rotation_limit);

            // Add velocity to rotation deltas
            yaw_delta += player.yaw_velocity * dt_factor;
            pitch_delta += player.pitch_velocity * dt_factor;

            // Apply exponential decay to velocities
            let decay = mouse_look.smooth_factor.powf(dt_factor);
            player.yaw_velocity *= decay;
            player.pitch_velocity *= decay;
        }

        // Apply rotation
        if yaw_delta != 0.0 || pitch_delta != 0.0 {
            // Apply yaw rotation around the world Z axis
            if yaw_delta != 0.0 {
                let yaw_rotation = Quat::from_axis_angle(Vec3::Z, yaw_delta);
                transform.rotation = yaw_rotation * transform.rotation;
            }

            // Apply pitch rotation around the local X axis (right vector)
            if pitch_delta != 0.0 {
                // Calculate current pitch from the forward vector's Z component
                let forward_3d = transform.forward().as_vec3();
                let current_pitch = f32::asin(forward_3d.z.clamp(-1.0, 1.0));

                // Calculate new pitch and clamp to limits (from MouseLookSettings)
                let new_pitch = (current_pitch + pitch_delta)
                    .clamp(-mouse_look.pitch_limit, mouse_look.pitch_limit);
                let actual_pitch_delta = new_pitch - current_pitch;

                // Apply the pitch rotation around the local right (X) axis
                if actual_pitch_delta.abs() > 0.0001 {
                    let local_x = transform.right().as_vec3();
                    let pitch_rotation = Quat::from_axis_angle(local_x, actual_pitch_delta);
                    transform.rotation = pitch_rotation * transform.rotation;
                }
            }
        }

        // Apply XY plane movement in camera's local orientation (projected to XY plane)
        if movement_xy != Vec2::ZERO {
            movement_xy = movement_xy.normalize();

            // Get forward and right directions, but project them onto the XY plane
            let forward_3d = transform.forward();
            let right_3d = transform.right();

            // Project to XY plane by zeroing Z component and normalizing
            let forward_xy = Vec2::new(forward_3d.x, forward_3d.y).normalize_or_zero();
            let right_xy = Vec2::new(right_3d.x, right_3d.y).normalize_or_zero();

            let move_vec_xy = forward_xy * movement_xy.y + right_xy * movement_xy.x;

            // Calculate new position
            let new_x = transform.translation.x + move_vec_xy.x * player.speed * dt;
            let new_y = transform.translation.y + move_vec_xy.y * player.speed * dt;

            // Check collision before moving
            if map.can_move_to(new_x, new_y, PLAYER_RADIUS) {
                transform.translation.x = new_x;
                transform.translation.y = new_y;
            }
        }

        // Apply Z axis movement (no collision check for vertical movement)
        if movement_z != 0.0 {
            transform.translation.z += movement_z * player.speed * dt;
        }
    }
}

pub fn update_player_light(
    player_query: Query<&Transform, With<Player>>,
    mut light_query: Query<(&mut Transform, &PlayerLight), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        // Update all lights using their offsets
        for (mut light_transform, player_light) in light_query.iter_mut() {
            light_transform.translation = player_transform.translation + player_light.offset;
        }
    }
}

fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255) as f32 / 255.0;

    Color::srgb(r, g, b)
}

pub fn update_player_light_animation(
    time: Res<Time>,
    mut light_query: Query<(&mut PointLight, &mut LightColorAnimation), With<PlayerLight>>,
) {
    for (mut light, mut anim) in light_query.iter_mut() {
        let dt = time.delta_secs();
        anim.time += 0.1 * dt * anim.speed;

        let light_yellow = hex_to_color("#f0ead5ff");
        let red = hex_to_color("#eac2acff");
        let yellow_white = hex_to_color("#dfac99ff");

        // Create a smooth oscillation through the three colors
        // Use sine wave that goes 0 -> 1 -> 2 -> 1 -> 0 (one full cycle)
        let t = (anim.time * std::f32::consts::PI).sin().abs();

        // Map t (0 to 1) to blend between the three colors
        let color = if t < 0.5 {
            // Blend from light_yellow to red
            let blend = t * 2.0; // 0 to 1
            Color::srgb(
                light_yellow.to_srgba().red * (1.0 - blend) + red.to_srgba().red * blend,
                light_yellow.to_srgba().green * (1.0 - blend) + red.to_srgba().green * blend,
                light_yellow.to_srgba().blue * (1.0 - blend) + red.to_srgba().blue * blend,
            )
        } else {
            // Blend from red to yellow_white
            let blend = (t - 0.5) * 2.0; // 0 to 1
            Color::srgb(
                red.to_srgba().red * (1.0 - blend) + yellow_white.to_srgba().red * blend,
                red.to_srgba().green * (1.0 - blend) + yellow_white.to_srgba().green * blend,
                red.to_srgba().blue * (1.0 - blend) + yellow_white.to_srgba().blue * blend,
            )
        };

        light.color = color;

        // When we complete a cycle, randomize the speed for next cycle (+/- 20%)
        if anim.time >= 2.0 {
            anim.time = 0.0;
            let mut rng = rand::rng();
            anim.speed = 1.0 + rng.random_range(-0.2..0.2);
        }
    }
}

/// Spawn camera at given position and return its entity ID
pub fn spawn_camera(commands: &mut Commands, position: Vec3) -> Entity {
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(position.x, position.y, position.z).looking_at(
                Vec3::new(position.x - 1.0, position.y, position.z * 1.01),
                Vec3::Z,
            ),
            Player {
                speed: 32.0,
                yaw_velocity: 0.0,
                pitch_velocity: 0.0,
            },
        ))
        .id()
}

/// Spawn player lights that follow the camera
pub fn spawn_player_lights(commands: &mut Commands, position: Vec3) {
    // Add a point light that follows the player
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 1000000.0,
            range: 64.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(position.x + 0.0, position.y + 1.5, position.z + 4.0),
        PlayerLight {
            offset: Vec3::new(0.0, 1.5, 4.0),
        },
        LightColorAnimation::new(0.0, 0.1),
    ));

    // Add a second point light that follows the player with no Y offset
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 1000000.0,
            range: 64.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(position.x + 0.5, position.y - 0.5, position.z + 4.0),
        PlayerLight {
            offset: Vec3::new(0.5, -0.5, 4.0),
        },
        LightColorAnimation::new(0.5, 0.2),
    ));
}
