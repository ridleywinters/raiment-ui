use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::FRAC_PI_2;

const PLAYER_LIGHT_OFFSET: f32 = 4.0;

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
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                camera_control_system,
                update_player_light,
                animate_player_light,
                configure_stone_texture,
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
struct PlayerLight;

#[derive(Component)]
struct LightColorAnimation {
    time: f32,
    speed: f32,
}

impl Default for LightColorAnimation {
    fn default() -> Self {
        Self {
            time: 0.0,
            speed: 1.0,
        }
    }
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // Create a checker pattern texture
    let checker_image = create_checker_texture(512, 512);
    let checker_texture = images.add(checker_image);

    // Create a 512x512 plane in the XY plane at z=0
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(512.0, 512.0));
    let plane_material = materials.add(StandardMaterial {
        base_color_texture: Some(checker_texture),
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        metallic: 0.0,
        reflectance: 0.0,
        uv_transform: bevy::math::Affine2::from_scale(Vec2::new(4.0, 4.0)),
        ..default()
    });
    let plane_material2 = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("base/textures/stone.png")),
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
            .translated_by(Vec3::new(0.0, 0.0, 4.0)),
    );

    let cube_mesh2 = meshes.add(
        Cuboid::new(8.0, 8.0, 16.0)
            .mesh()
            .build()
            .translated_by(Vec3::new(0.0, 0.0, 8.0)),
    );

    // PICO-8 palette colors
    let pico8_colors = [
        Color::srgb(0.0, 0.0, 0.0),    // Black
        Color::srgb(0.11, 0.17, 0.33), // Dark blue
        Color::srgb(0.49, 0.15, 0.35), // Dark purple
        Color::srgb(0.0, 0.53, 0.33),  // Dark green
        Color::srgb(0.67, 0.32, 0.21), // Brown
        Color::srgb(0.37, 0.35, 0.31), // Dark gray
        Color::srgb(0.76, 0.76, 0.78), // Light gray
        Color::srgb(1.0, 0.95, 0.91),  // White
        Color::srgb(1.0, 0.0, 0.3),    // Red
        Color::srgb(1.0, 0.64, 0.0),   // Orange
        Color::srgb(1.0, 0.95, 0.27),  // Yellow
        Color::srgb(0.0, 0.89, 0.21),  // Green
        Color::srgb(0.16, 0.67, 1.0),  // Blue
        Color::srgb(0.51, 0.46, 0.61), // Indigo
        Color::srgb(1.0, 0.47, 0.77),  // Pink
        Color::srgb(1.0, 0.8, 0.67),   // Peach
    ];

    // Load map from data/map.txt
    let map_content = std::fs::read_to_string("data/map.txt").expect("Failed to read data/map.txt");

    let mut rng = rand::rng();

    // Parse the map and create cubes for each 'X'
    for (row, line) in map_content.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let mesh = match ch {
                'X' => cube_mesh2.clone(),
                'x' => cube_mesh.clone(),
                _ => continue,
            };

            // Position: each cell is 8x8, so multiply by 8
            let x = col as f32 * 8.0;
            let y = row as f32 * 8.0;

            // Pick a random PICO-8 color
            let color = pico8_colors[rng.random_range(0..pico8_colors.len())];

            commands.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(materials.add(color)),
                Transform::from_translation(Vec3::new(x, y, 0.0)),
            ));
        }
    }

    commands.insert_resource(bevy::light::AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
        affects_lightmapped_meshes: false,
    });

    let player_start_pos = Vec3::new(256.0, 206.0, 6.4);

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(player_start_pos.x, player_start_pos.y, player_start_pos.z)
            .looking_at(Vec3::new(-2092.0, 344.0, 6.4), Vec3::Z),
        Player {
            speed: 50.0,
            rot_speed: 1.5,
        },
    ));

    // Add a point light that follows the player
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 5000000.0,
            range: 64.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(
            player_start_pos.x,
            player_start_pos.y,
            player_start_pos.z + PLAYER_LIGHT_OFFSET,
        ),
        PlayerLight, // Marker component to identify this light
        LightColorAnimation::default(),
    ));
}

fn camera_control_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in query.iter_mut() {
        let dt = time.delta_secs();

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

        // Movement input (WASD + QE)
        // WASD moves in the XY plane, Q/E moves along Z axis
        let mut movement_xy = Vec2::ZERO; // Movement in XY plane
        let mut movement_z = 0.0; // Movement along Z axis

        if input.pressed(KeyCode::KeyW) {
            movement_xy.y += 1.0; // Forward
        }
        if input.pressed(KeyCode::KeyS) {
            movement_xy.y -= 1.0; // Backward
        }
        if input.pressed(KeyCode::KeyA) {
            movement_xy.x -= 1.0; // Left
        }
        if input.pressed(KeyCode::KeyD) {
            movement_xy.x += 1.0; // Right
        }
        if input.pressed(KeyCode::KeyF) {
            movement_z -= 1.0; // Down
        }
        if input.pressed(KeyCode::KeyR) {
            movement_z += 1.0; // Up
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
            transform.translation.x += move_vec_xy.x * player.speed * dt;
            transform.translation.y += move_vec_xy.y * player.speed * dt;
        }

        // Apply Z axis movement
        if movement_z != 0.0 {
            transform.translation.z += movement_z * player.speed * dt;
        }
    }
}

fn update_player_light(
    player_query: Query<&Transform, With<Player>>,
    mut light_query: Query<&mut Transform, (With<PlayerLight>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut light_transform) = light_query.single_mut() {
            // Position the light slightly above the player
            light_transform.translation =
                player_transform.translation + Vec3::new(0.0, 0.0, PLAYER_LIGHT_OFFSET);
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

fn animate_player_light(
    time: Res<Time>,
    mut light_query: Query<(&mut PointLight, &mut LightColorAnimation), With<PlayerLight>>,
) {
    if let Ok((mut light, mut anim)) = light_query.single_mut() {
        let dt = time.delta_secs();
        anim.time += 0.1 * dt * anim.speed;

        let light_yellow = hex_to_color("#e8d599");
        let red = hex_to_color("#ffb96e");
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

fn configure_stone_texture(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut configured: Local<bool>,
) {
    // Only run once
    if *configured {
        return;
    }

    // Check if the stone texture is loaded
    let handle: Handle<Image> = asset_server.load("base/textures/stone.png");
    if let Some(image) = images.get_mut(&handle) {
        // Configure the sampler for repeat mode and nearest filtering
        image.sampler =
            bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                address_mode_u: bevy::image::ImageAddressMode::Repeat,
                address_mode_v: bevy::image::ImageAddressMode::Repeat,
                mag_filter: bevy::image::ImageFilterMode::Nearest,
                min_filter: bevy::image::ImageFilterMode::Nearest,
                ..Default::default()
            });
        *configured = true;
    }
}

fn create_checker_texture(width: u32, height: u32) -> Image {
    use bevy::asset::RenderAssetUsages;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            // Create checkerboard pattern (8x8 pixel squares)
            let checker_size = 4;
            let is_white = ((x / checker_size) + (y / checker_size)) % 2 == 0;

            let color = if is_white {
                [220, 220, 220, 255] // Light gray
            } else {
                [80, 80, 80, 255] // Dark gray
            };

            data.extend_from_slice(&color);
        }
    }

    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    // Set nearest filtering with repeat mode for tiling
    image.sampler = bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
        address_mode_u: bevy::image::ImageAddressMode::Repeat,
        address_mode_v: bevy::image::ImageAddressMode::Repeat,
        mag_filter: bevy::image::ImageFilterMode::Nearest,
        min_filter: bevy::image::ImageFilterMode::Nearest,
        ..Default::default()
    });

    image
}
