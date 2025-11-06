/// Audio feedback for combat actions
///
/// Handles sound effects for weapon swings, hits, and other combat events.
use bevy::prelude::*;

/// Resource containing audio handles for combat sounds
#[derive(Resource, Debug)]
pub struct CombatAudio {
    /// Sound played when swinging a weapon
    pub swing_sound: Option<Handle<AudioSource>>,

    /// Sound played when hitting an enemy
    pub hit_sound: Option<Handle<AudioSource>>,

    /// Sound played for critical hits
    pub critical_sound: Option<Handle<AudioSource>>,
}

impl CombatAudio {
    /// Create a new CombatAudio resource (sounds will be loaded separately)
    pub fn new() -> Self {
        Self {
            swing_sound: None,
            hit_sound: None,
            critical_sound: None,
        }
    }

    /// Load combat sounds from asset server
    pub fn load_sounds(asset_server: &Res<AssetServer>) -> Self {
        Self {
            swing_sound: Some(asset_server.load("base/sounds/swing1.ogg")),
            hit_sound: Some(asset_server.load("base/sounds/hit1.ogg")),
            critical_sound: Some(asset_server.load("base/sounds/critical1.ogg")),
        }
    }
}

impl Default for CombatAudio {
    fn default() -> Self {
        Self::new()
    }
}

/// Play a swing sound effect
pub fn play_swing_sound(commands: &mut Commands, combat_audio: &Res<CombatAudio>) {
    if let Some(sound) = &combat_audio.swing_sound {
        commands.spawn((AudioPlayer::new(sound.clone()), PlaybackSettings::DESPAWN));
    }
}

/// Play a hit sound effect
pub fn play_hit_sound(commands: &mut Commands, combat_audio: &Res<CombatAudio>, critical: bool) {
    let sound = if critical {
        &combat_audio.critical_sound
    } else {
        &combat_audio.hit_sound
    };

    if let Some(sound) = sound {
        commands.spawn((AudioPlayer::new(sound.clone()), PlaybackSettings::DESPAWN));
    }
}
