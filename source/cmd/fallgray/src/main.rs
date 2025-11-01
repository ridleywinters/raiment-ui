mod collision;
mod texture_loader;
mod ui;

use bevy::prelude::*;
use collision::{CollisionMap, PLAYER_RADIUS, check_circle_collision};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::f32::consts::FRAC_PI_2;
use texture_loader::{load_image_texture, load_weapon_texture};
use ui::*;

#[derive(Serialize, Deserialize)]
pub struct ItemDefinition {
    pub image: String,
    pub script: String,
    pub scale: f32,
    pub effects: Vec<String>,
}

#[derive(Deserialize)]
struct ItemDefinitionsFile {
    items: HashMap<String, ItemDefinition>,
}

#[derive(Resource)]
struct ItemDefinitions {
    items: HashMap<String, ItemDefinition>,
}

#[derive(Deserialize, Serialize)]
struct MapFile {
    map: MapData,
}

#[derive(Deserialize, Serialize)]
struct MapData {
    grid: Vec<String>,
    #[serde(default)]
    items: Vec<ItemPosition>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct ItemPosition {
    x: f32,
    y: f32,
    #[serde(default = "default_item_type")]
    item_type: String,
}

fn default_item_type() -> String {
    "apple".to_string()
}

#[derive(Resource)]
struct ItemTracker {
    positions: HashSet<(i32, i32)>, // Grid positions where items exist
    world_positions: Vec<(f32, f32, String)>, // Actual world positions and item types for saving
}

impl Default for ItemTracker {
    fn default() -> Self {
        Self {
            positions: HashSet::new(),
            world_positions: Vec::new(),
        }
    }
}

impl ItemTracker {
    fn remove_at_position(&mut self, world_x: f32, world_y: f32) {
        let grid_x = (world_x / 8.0).floor() as i32;
        let grid_y = (world_y / 8.0).floor() as i32;
        self.positions.remove(&(grid_x, grid_y));
        self.world_positions
            .retain(|(x, y, _)| (*x - world_x).abs() > 0.1 || (*y - world_y).abs() > 0.1);
    }
}

fn main() {
    // Get asset path from REPO_ROOT environment variable
    let asset_path = std::env::var("REPO_ROOT")
        .map(|repo_root| format!("{}/source/assets", repo_root))
        .unwrap_or_else(|_| "assets".to_string());

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(bevy::asset::AssetPlugin {
                    file_path: asset_path,
                    ..default()
                })
                .set(bevy::window::WindowPlugin {
                    primary_window: Some(bevy::window::Window {
                        title: "Fallgray".into(),
                        resolution: (1920, 1080).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(
            Startup,
            (
                startup_system, //
                startup_ui,
            ),
        )
        .add_systems(
            Update,
            (
                update_camera_control_system,
                update_player_light,
                update_player_light_animation,
                update_weapon_swing,
                update_ui,
                update_toolbar_input,
                update_toolbar_click,
                update_billboards,
                update_spawn_item_on_click,
                update_save_map_on_input,
                update_check_item_collision,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
    rot_speed: f32,
}

#[derive(Component)]
struct PlayerLight {
    offset: Vec3,
}

#[derive(Component)]
struct Billboard;

#[derive(Component)]
struct Item {
    interaction_radius: f32,
}

#[derive(Component)]
struct GroundPlane;

#[derive(Component)]
struct LightColorAnimation {
    time: f32,
    speed: f32,
}

// Weapon swing components
#[derive(Component)]
struct WeaponSprite {
    swing_timer: Timer,
    is_swinging: bool,
}

impl Default for WeaponSprite {
    fn default() -> Self {
        Self {
            swing_timer: Timer::from_seconds(0.4, TimerMode::Once),
            is_swinging: false,
        }
    }
}

// ===== WEAPON ANIMATION CONSTANTS =====

// Animation timing phases
const WINDUP_END: f32 = 0.15; // 15% - Wind-up phase
const SWING_END: f32 = 0.50; // 50% - Thrust/swing phase (35% duration)
const FOLLOWTHROUGH_END: f32 = 1.0; // 100% - Follow-through phase (50% duration)

// Rest position (idle state)
const REST_POS_X: f32 = 0.6; // Right side of screen
const REST_POS_Y: f32 = -0.45; // Lower on screen
const REST_POS_Z: f32 = -1.2; // Distance from camera
const REST_ROTATION_Z: f32 = 0.0; // No spin at rest
const REST_ROTATION_Y: f32 = 0.0; // No tilt at rest

// Wind-up position
const WINDUP_POS_X: f32 = 0.7; // Slightly more right
const WINDUP_POS_Y: f32 = -0.35; // Slightly higher
const WINDUP_POS_Z: f32 = -0.8; // Pull back toward camera
const WINDUP_ROTATION_Z: f32 = -0.5; // Counter-clockwise wind-up
const WINDUP_ROTATION_Y: f32 = 0.8; // Tilt right

// Thrust end position
const THRUST_POS_X: f32 = 0.3; // Move toward center
const THRUST_POS_Y: f32 = -0.45; // Slightly higher than rest
const THRUST_POS_Z: f32 = -1.5; // Extend forward
const THRUST_ROTATION_Z: f32 = 1.55; // Large clockwise spin (~89°)
const THRUST_ROTATION_Y: f32 = -1.3; // Tilt left (~-74°)

impl Default for LightColorAnimation {
    fn default() -> Self {
        Self {
            time: 0.0,
            speed: 1.0,
        }
    }
}

// Easing functions for weapon swing
fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Create a 512x512 plane in the XY plane at z=0
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(512.0, 512.0));
    let plane_material2 = materials.add(StandardMaterial {
        base_color_texture: Some(load_image_texture(
            &asset_server,
            "base/textures/stone_1.png",
        )),
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        metallic: 0.0,
        reflectance: 0.0,
        uv_transform: bevy::math::Affine2::from_scale(Vec2::new(64.0, 64.0)),
        ..default()
    });

    commands.spawn((
        Mesh3d(plane_mesh.clone()),
        MeshMaterial3d(plane_material2.clone()),
        // Rotate 90 degrees around X to make it XY plane (facing Z)
        Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2))
            .with_translation(Vec3::new(256.0, 256.0, 0.0)),
        GroundPlane,
    ));

    commands.spawn((
        Mesh3d(plane_mesh.clone()),
        MeshMaterial3d(plane_material2.clone()),
        Transform::from_rotation(Quat::from_rotation_x(3.0 * FRAC_PI_2))
            .with_translation(Vec3::new(256.0, 256.0, 16.0)),
    ));

    // Add some 8x8x8 cubes as reference points
    // Translate the mesh by +4.0 in Z so cubes sit on the ground plane
    let cube_mesh = meshes.add(
        Cuboid::new(8.0, 8.0, 8.0)
            .mesh()
            .build()
            .translated_by(Vec3::new(4.0, 4.0, 4.0)),
    );

    let cube_mesh2 = meshes.add(
        Cuboid::new(8.0, 8.0, 16.0)
            .mesh()
            .build()
            .translated_by(Vec3::new(4.0, 4.0, 8.0)),
    );

    // Load map from data/map.yaml
    let map_yaml = std::fs::read_to_string("data/map.yaml").expect("Failed to read data/map.yaml");
    let map_file: MapFile = serde_yaml::from_str(&map_yaml).expect("Failed to parse map.yaml");
    let lines = map_file.map.grid;

    // Load item definitions from data/item_definitions.yaml
    let filename = std::env::var("REPO_ROOT")
        .map(|repo_root| format!("{}/source/assets/base/items/items.yaml", repo_root))
        .unwrap_or_else(|_| "data/item_definitions.yaml".to_string());
    let item_defs_yaml =
        std::fs::read_to_string(&filename).expect(&format!("Failed to read {}", filename));
    let item_defs_file: ItemDefinitionsFile =
        serde_yaml::from_str(&item_defs_yaml).expect(&format!("Failed to parse {}", filename));
    let item_definitions = ItemDefinitions {
        items: item_defs_file.items,
    };

    // Build collision map
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut collision_grid = HashMap::new();

    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(load_image_texture(
            &asset_server,
            "base/textures/stone_2.png",
        )),
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        metallic: 0.0,
        reflectance: 0.0,
        uv_transform: bevy::math::Affine2::from_scale(Vec2::new(1.0, 1.0)),
        ..default()
    });

    // Parse the map and create cubes for each 'X'
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            // Mark filled cells in collision grid
            let is_solid = matches!(ch, 'X' | 'x');
            if is_solid {
                collision_grid.insert((col as i32, row as i32), true);
            }

            // Position: each cell is 8x8, so multiply by 8
            let x = col as f32 * 8.0;
            let y = row as f32 * 8.0;

            match ch {
                'X' => {
                    commands.spawn((
                        Mesh3d(cube_mesh2.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ));
                }
                'x' => {
                    commands.spawn((
                        Mesh3d(cube_mesh.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ));
                }
                'c' => {
                    // Spawn a billboarded NPC sprite
                    let scale = 3.8;
                    spawn_billboard_sprite(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &asset_server,
                        Vec3::new(x + 4.0, y + 4.0, scale),
                        "base/sprites/monster-skeleton-01.png",
                        scale,
                    );
                }
                _ => {}
            }
        }
    }

    // Insert collision map as a resource
    commands.insert_resource(CollisionMap::new(collision_grid, width, height));

    // Initialize item tracker and spawn existing items
    let mut item_tracker = ItemTracker::default();

    for item_pos in &map_file.map.items {
        // Track the item position
        let grid_x = (item_pos.x / 8.0).floor() as i32;
        let grid_y = (item_pos.y / 8.0).floor() as i32;
        item_tracker.positions.insert((grid_x, grid_y));
        item_tracker
            .world_positions
            .push((item_pos.x, item_pos.y, item_pos.item_type.clone()));

        // Get scale from item definition for positioning
        let item_def = item_definitions
            .items
            .get(&item_pos.item_type)
            .expect(&format!(
                "Item definition not found: {}",
                item_pos.item_type
            ));
        let scale = item_def.scale;

        // Spawn the item billboard
        spawn_item(
            &mut commands,
            &mut meshes,
            &mut materials,
            &asset_server,
            &item_definitions.items,
            Vec3::new(item_pos.x, item_pos.y, scale),
            &item_pos.item_type,
        );
    }

    commands.insert_resource(item_tracker);
    commands.insert_resource(item_definitions);

    commands.insert_resource(bevy::light::AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
        affects_lightmapped_meshes: false,
    });

    let player_start_pos = Vec3::new(256.0 + 4.0, 200.0 + 4.0, 4.8);

    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(player_start_pos.x, player_start_pos.y, player_start_pos.z)
                .looking_at(
                    Vec3::new(
                        player_start_pos.x - 1.0,
                        player_start_pos.y,
                        player_start_pos.z * 1.01,
                    ),
                    Vec3::Z,
                ),
            Player {
                speed: 32.0,
                rot_speed: 2.75,
            },
        ))
        .id();

    // Spawn weapon sprite as child of camera for first-person view
    spawn_weapon_sprite(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        camera_entity,
    );

    // Add a point light that follows the player
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 1000000.0,
            range: 64.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(
            player_start_pos.x + 0.0,
            player_start_pos.y + 1.5,
            player_start_pos.z + 4.0,
        ),
        PlayerLight {
            offset: Vec3::new(0.0, 1.5, 4.0),
        },
        LightColorAnimation::default(),
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
        Transform::from_xyz(
            player_start_pos.x + 0.5,
            player_start_pos.y - 0.5,
            player_start_pos.z + 4.0,
        ),
        PlayerLight {
            offset: Vec3::new(0.5, -0.5, 4.0),
        },
    ));
}

fn update_billboards(
    camera_query: Query<&Transform, With<Camera3d>>,
    mut billboard_query: Query<&mut Transform, (With<Billboard>, Without<Camera3d>)>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_pos = camera_transform.translation;

        for mut billboard_transform in billboard_query.iter_mut() {
            let billboard_pos = billboard_transform.translation;

            // Calculate direction from billboard to camera (in XY plane)
            let to_camera = Vec2::new(
                camera_pos.x - billboard_pos.x,
                camera_pos.y - billboard_pos.y,
            );

            // The plane's normal starts pointing in X direction (Dir3::X)
            // Calculate angle around Z axis to rotate the normal to face the camera
            let angle = to_camera.y.atan2(to_camera.x);

            // Rotate around Z axis so the plane normal points toward camera
            billboard_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

fn update_camera_control_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    collision_map: Res<CollisionMap>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in query.iter_mut() {
        let dt = time.delta_secs();

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
        let mut yaw_delta = 0.0;
        let mut pitch_delta = 0.0;

        if input.pressed(KeyCode::ArrowLeft) {
            yaw_delta += player.rot_speed * dt;
        }
        if input.pressed(KeyCode::ArrowRight) {
            yaw_delta -= player.rot_speed * dt;
        }
        if input.pressed(KeyCode::ArrowUp) {
            pitch_delta += player.rot_speed * dt;
        }
        if input.pressed(KeyCode::ArrowDown) {
            pitch_delta -= player.rot_speed * dt;
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

            let max = scale * player.rot_speed * dt;
            yaw_delta += (yaw_snap - yaw).clamp(-max, max);
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

                // Calculate new pitch and clamp to limits
                let pitch_limit = 70_f32.to_radians();
                let new_pitch = (current_pitch + pitch_delta).clamp(-pitch_limit, pitch_limit);
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
            if collision_map.can_move_to(new_x, new_y, PLAYER_RADIUS) {
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

#[allow(clippy::type_complexity)]
fn update_player_light(
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

fn update_player_light_animation(
    time: Res<Time>,
    mut light_query: Query<(&mut PointLight, &mut LightColorAnimation), With<PlayerLight>>,
) {
    if let Ok((mut light, mut anim)) = light_query.single_mut() {
        let dt = time.delta_secs();
        anim.time += 0.1 * dt * anim.speed;

        let light_yellow = hex_to_color("#e8d599");
        let red = hex_to_color("#e7844fff");
        let yellow_white = hex_to_color("#e4bb6f");

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

fn update_weapon_swing(
    time: Res<Time>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    toolbar: Res<Toolbar>,
    mut weapon_query: Query<(&mut Transform, &mut WeaponSprite, &mut Visibility)>,
    ui_interaction_query: Query<&Interaction>,
) {
    for (mut transform, mut weapon, mut visibility) in weapon_query.iter_mut() {
        // Only show the weapon sprite when slot 1 is active
        *visibility = if toolbar.active_slot == 1 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        // Check for attack input (left mouse button) - only swing if slot 1 is active
        if mouse_button.just_pressed(MouseButton::Left)
            && !weapon.is_swinging
            && toolbar.active_slot == 1
        {
            // Check if any UI element is being interacted with
            let ui_blocked = ui_interaction_query
                .iter()
                .any(|interaction| *interaction != Interaction::None);
            if !ui_blocked {
                weapon.is_swinging = true;
                weapon.swing_timer.reset();
            }
        }

        if weapon.is_swinging {
            weapon.swing_timer.tick(time.delta());
            let progress = weapon.swing_timer.fraction();

            let rest_pos = Vec3::new(REST_POS_X, REST_POS_Y, REST_POS_Z);
            let rest_rotation_z = REST_ROTATION_Z;
            let rest_rotation_y = REST_ROTATION_Y;

            // Calculate current position and rotation based on phase
            let (current_pos, current_rotation_z, current_rotation_y) = if progress < WINDUP_END {
                // Wind-up phase: pull back toward camera
                let phase_t = progress / WINDUP_END;
                let eased_t = ease_out_quad(phase_t);

                let windup_pos = Vec3::new(WINDUP_POS_X, WINDUP_POS_Y, WINDUP_POS_Z);

                (
                    rest_pos.lerp(windup_pos, eased_t),
                    REST_ROTATION_Z + (WINDUP_ROTATION_Z - REST_ROTATION_Z) * eased_t,
                    REST_ROTATION_Y + (WINDUP_ROTATION_Y - REST_ROTATION_Y) * eased_t,
                )
            } else if progress < SWING_END {
                // Thrust phase: fast FORWARD motion with rotation
                let phase_t = (progress - WINDUP_END) / (SWING_END - WINDUP_END);
                let eased_t = ease_in_out_cubic(phase_t);

                let windup_pos = Vec3::new(WINDUP_POS_X, WINDUP_POS_Y, WINDUP_POS_Z);
                let thrust_end_pos = Vec3::new(THRUST_POS_X, THRUST_POS_Y, THRUST_POS_Z);

                (
                    windup_pos.lerp(thrust_end_pos, eased_t),
                    WINDUP_ROTATION_Z + (THRUST_ROTATION_Z - WINDUP_ROTATION_Z) * eased_t,
                    WINDUP_ROTATION_Y + (THRUST_ROTATION_Y - WINDUP_ROTATION_Y) * eased_t,
                )
            } else {
                // Follow-through phase: deceleration back to rest
                let phase_t = (progress - SWING_END) / (FOLLOWTHROUGH_END - SWING_END);
                let eased_t = ease_out_quad(phase_t);

                let thrust_end_pos = Vec3::new(THRUST_POS_X, THRUST_POS_Y, THRUST_POS_Z);

                (
                    thrust_end_pos.lerp(rest_pos, eased_t),
                    THRUST_ROTATION_Z + (REST_ROTATION_Z - THRUST_ROTATION_Z) * eased_t,
                    THRUST_ROTATION_Y + (REST_ROTATION_Y - THRUST_ROTATION_Y) * eased_t,
                )
            };

            // Apply transforms - combine Z rotation and Y rotation (tilt)
            transform.translation = current_pos;
            transform.rotation = Quat::from_rotation_z(current_rotation_z)
                * Quat::from_rotation_y(current_rotation_y);

            // Check if animation is complete
            if weapon.swing_timer.is_finished() {
                weapon.is_swinging = false;
                transform.translation = rest_pos;
                transform.rotation =
                    Quat::from_rotation_z(rest_rotation_z) * Quat::from_rotation_y(rest_rotation_y);
            }
        }
    }
}

fn spawn_billboard_sprite(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    sprite_path: &str,
    scale: f32,
) {
    let sprite_material = materials.add(StandardMaterial {
        base_color_texture: Some(load_image_texture(asset_server, sprite_path)),
        base_color: Color::WHITE,
        alpha_mode: bevy::render::alpha::AlphaMode::Blend,
        unlit: false,
        cull_mode: None,
        ..default()
    });

    use bevy::asset::RenderAssetUsages;
    use bevy::mesh::{Indices, PrimitiveTopology};

    let mut billboard_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let positions = vec![
        [0.0, -scale, -scale], // bottom-left
        [0.0, scale, -scale],  // top-left
        [0.0, scale, scale],   // top-right
        [0.0, -scale, scale],  // bottom-right
    ];
    billboard_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    billboard_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; 4]);

    let uvs = vec![
        [0.0, 1.0], // top-left -> bottom-left in texture
        [1.0, 1.0], // top-right -> bottom-right in texture
        [1.0, 0.0], // bottom-right -> top-right in texture
        [0.0, 0.0], // bottom-left -> top-left in texture
    ];
    billboard_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    billboard_mesh.insert_indices(Indices::U32(vec![
        0, 1, 2, // first triangle
        0, 2, 3, // second triangle
    ]));

    commands.spawn((
        Mesh3d(meshes.add(billboard_mesh)),
        MeshMaterial3d(sprite_material),
        Transform::from_translation(position),
        Billboard,
    ));
}

fn spawn_weapon_sprite(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    camera_entity: Entity,
) {
    use bevy::asset::RenderAssetUsages;
    use bevy::mesh::{Indices, PrimitiveTopology};

    let sprite_material = materials.add(StandardMaterial {
        base_color_texture: Some(load_weapon_texture(asset_server, "base/icons/sword.png")),
        base_color: Color::WHITE,
        alpha_mode: bevy::render::alpha::AlphaMode::Blend,
        unlit: true, // Keep weapon bright and visible
        cull_mode: None,
        ..default()
    });

    let scale = 0.5; // Smaller scale for weapon icon

    let mut weapon_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // Create a quad for the weapon sprite
    let positions = vec![
        [-scale, -scale, 0.0], // bottom-left
        [scale, -scale, 0.0],  // bottom-right
        [scale, scale, 0.0],   // top-right
        [-scale, scale, 0.0],  // top-left
    ];
    weapon_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    weapon_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4]);

    let uvs = vec![
        [0.0, 1.0], // bottom-left
        [1.0, 1.0], // bottom-right
        [1.0, 0.0], // top-right
        [0.0, 0.0], // top-left
    ];
    weapon_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    weapon_mesh.insert_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));

    // Spawn weapon as child of camera
    // Position it to the right and lower on screen
    // Close to camera to ensure it renders on top
    let weapon_entity = commands
        .spawn((
            Mesh3d(meshes.add(weapon_mesh)),
            MeshMaterial3d(sprite_material),
            Transform::from_xyz(REST_POS_X, REST_POS_Y, REST_POS_Z), // Use constants to match animation rest position
            WeaponSprite::default(),
        ))
        .id();

    // Parent weapon to camera
    commands
        .entity(camera_entity)
        .add_children(&[weapon_entity]);
}

fn spawn_item(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    item_definitions: &HashMap<String, ItemDefinition>,
    position: Vec3,
    item_key: &str,
) {
    let item_def = item_definitions
        .get(item_key)
        .unwrap_or_else(|| panic!("Item definition not found: {}", item_key));

    let sprite_material = materials.add(StandardMaterial {
        base_color_texture: Some(load_image_texture(asset_server, &item_def.image)),
        base_color: Color::WHITE,
        alpha_mode: bevy::render::alpha::AlphaMode::Blend,
        unlit: false,
        cull_mode: None,
        ..default()
    });

    use bevy::asset::RenderAssetUsages;
    use bevy::mesh::{Indices, PrimitiveTopology};

    let scale = item_def.scale;

    let mut billboard_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let positions = vec![
        [0.0, -scale, -scale],
        [0.0, scale, -scale],
        [0.0, scale, scale],
        [0.0, -scale, scale],
    ];
    billboard_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    billboard_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; 4]);

    let uvs = vec![[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
    billboard_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    billboard_mesh.insert_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));

    commands.spawn((
        Mesh3d(meshes.add(billboard_mesh)),
        MeshMaterial3d(sprite_material),
        Transform::from_translation(position),
        Billboard,
        Item {
            interaction_radius: 2.0,
        },
    ));
}

fn update_spawn_item_on_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&bevy::window::Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    ground_query: Query<&GlobalTransform, With<GroundPlane>>,
    ui_interaction_query: Query<&Interaction>,
    mut item_tracker: ResMut<ItemTracker>,
    toolbar: Res<Toolbar>,
    item_definitions: Res<ItemDefinitions>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Only spawn items if slot 2 or 3 is active
    if toolbar.active_slot != 2 && toolbar.active_slot != 3 {
        return;
    }

    // Check if any UI element is being interacted with
    for interaction in ui_interaction_query.iter() {
        if *interaction != Interaction::None {
            return;
        }
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok(_ground_transform) = ground_query.single() else {
        return;
    };

    // Convert cursor position to a ray in world space
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // The ground plane is at z=0 in world space (XY plane)
    // Ray equation: point = origin + t * direction
    // For intersection with z=0 plane: origin.z + t * direction.z = 0
    // Therefore: t = -origin.z / direction.z

    if ray.direction.z.abs() < 0.001 {
        // Ray is parallel to ground plane
        return;
    }

    let t = -ray.origin.z / ray.direction.z;

    if t < 0.0 {
        // Intersection is behind the camera
        return;
    }

    let intersection = ray.origin + ray.direction * t;

    // Force to a grid of 2x2 units
    let grid_x = (intersection.x / 2.0).floor() as i32;
    let grid_y = (intersection.y / 2.0).floor() as i32;

    // Check if there's already an item at this position
    if item_tracker.positions.contains(&(grid_x, grid_y)) {
        return;
    }

    // Calculate world position
    let world_x = grid_x as f32 * 2.0 + 1.0;
    let world_y = grid_y as f32 * 2.0 + 1.0;

    // Select item based on active toolbar slot
    let item_key = match toolbar.active_slot {
        2 => "apple",
        3 => "coin-gold",
        _ => "apple", // Fallback (shouldn't happen due to earlier check)
    };

    // Track the item
    item_tracker.positions.insert((grid_x, grid_y));
    item_tracker
        .world_positions
        .push((world_x, world_y, item_key.to_string()));

    // Get scale from item definition for positioning
    let item_def = item_definitions
        .items
        .get(item_key)
        .expect("Item definition not found");
    let scale = item_def.scale;

    // Spawn item billboard at the intersection point
    spawn_item(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        &item_definitions.items,
        Vec3::new(world_x, world_y, scale),
        item_key,
    );
}

fn update_save_map_on_input(input: Res<ButtonInput<KeyCode>>, item_tracker: Res<ItemTracker>) {
    // Press Ctrl+S to save the map
    if (input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight))
        && input.just_pressed(KeyCode::KeyS)
    {
        // Read the current map file
        let map_yaml = match std::fs::read_to_string("data/map.yaml") {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read map.yaml: {}", e);
                return;
            }
        };

        let mut map_file: MapFile = match serde_yaml::from_str(&map_yaml) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to parse map.yaml: {}", e);
                return;
            }
        };

        // Update items in the map data
        map_file.map.items = item_tracker
            .world_positions
            .iter()
            .map(|(x, y, item_type)| ItemPosition {
                x: *x,
                y: *y,
                item_type: item_type.clone(),
            })
            .collect();

        // Serialize and save
        let yaml_output = match serde_yaml::to_string(&map_file) {
            Ok(yaml) => yaml,
            Err(e) => {
                eprintln!("Failed to serialize map: {}", e);
                return;
            }
        };

        if let Err(e) = std::fs::write("data/map.yaml", yaml_output) {
            eprintln!("Failed to write map.yaml: {}", e);
        } else {
            println!(
                "Map saved successfully with {} items!",
                item_tracker.world_positions.len()
            );
        }
    }
}

fn update_check_item_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    item_query: Query<(Entity, &Transform, &Item)>,
    mut stats: ResMut<PlayerStats>,
    mut item_tracker: ResMut<ItemTracker>,
    item_definitions: Res<ItemDefinitions>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation;

    for (entity, item_transform, item) in item_query.iter() {
        let item_pos = item_transform.translation;

        if check_circle_collision(player_pos, item_pos, item.interaction_radius) {
            // Find the item type from the tracker
            let item_type = item_tracker
                .world_positions
                .iter()
                .find(|(x, y, _)| (*x - item_pos.x).abs() < 0.1 && (*y - item_pos.y).abs() < 0.1)
                .map(|(_, _, item_type)| item_type.as_str())
                .unwrap_or("apple");

            // Get the item definition and print the script
            if let Some(item_def) = item_definitions.items.get(item_type) {
                println!("Item script: {}", item_def.script);
                process_script(&item_def.script, &mut stats);
            }

            // Remove item from world
            commands.entity(entity).despawn();

            // Remove from tracker
            item_tracker.remove_at_position(item_pos.x, item_pos.y);

            println!("Collected item! Fatigue: {}", stats.stamina);
        }
    }
}

fn process_script(script: &str, stats: &mut ResMut<PlayerStats>) {
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        // Skip comment lines
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        let words: Vec<&str> = trimmed.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }

        match words[0] {
            "add_gold" => {
                if words.len() >= 2 {
                    if let Ok(amount) = words[1].parse::<i32>() {
                        stats.gold += amount;
                        println!("Added {} gold, new value: {}", amount, stats.gold);
                    } else {
                        eprintln!("Invalid gold amount: {}", words[1]);
                    }
                } else {
                    eprintln!("add_gold requires an amount");
                }
            }
            "add_stamina" => {
                if words.len() >= 2 {
                    if let Ok(amount) = words[1].parse::<f32>() {
                        stats.stamina = (stats.stamina + amount).min(100.0);
                        println!("Added {} stamina, new value: {}", amount, stats.stamina);
                    } else {
                        eprintln!("Invalid stamina amount: {}", words[1]);
                    }
                } else {
                    eprintln!("add_stamina requires an amount");
                }
            }
            _ => {
                eprintln!("Unknown command: {}", words.join(" "));
            }
        }
    }
}
