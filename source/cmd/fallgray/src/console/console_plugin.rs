use super::console_ui::*;
use bevy::prelude::*;

//=============================================================================
// Console Plugin
//=============================================================================

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, startup_console)
            .add_systems(
                Update,
                (
                    update_console_toggle,
                    update_console_input,
                    update_console_scroll,
                ),
            );
    }
}
