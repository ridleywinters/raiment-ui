use super::cursor_toggle::*;
use super::mouse_look_settings::MouseLookSettings;
use super::systems::*;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseLookSettings>()
            .add_systems(
                Update,
                (
                    toggle_cursor_lock,
                    click_to_lock_cursor,
                    handle_console_cursor,
                    update_camera_control_system,
                    update_player_light,
                    update_player_light_animation,
                ),
            );
    }
}
