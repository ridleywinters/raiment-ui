use super::cvars::CVarRegistry;
use crate::actor::Actor;
use crate::ui::PlayerStats;
use bevy::prelude::*;

use super::cmd_add_gold::cmd_add_gold;
use super::cmd_add_stamina::cmd_add_stamina;
use super::cmd_do_damage::cmd_do_damage;
use super::cmd_getvar::cmd_getvar;
use super::cmd_listvars::cmd_listvars;
use super::cmd_quit::cmd_quit;
use super::cmd_savecvars::cmd_savecvars;
use super::cmd_setvar::cmd_setvar;

pub fn process_script(
    script: &str,
    stats: &mut ResMut<PlayerStats>,
    cvars: &mut ResMut<CVarRegistry>,
) -> Vec<String> {
    process_script_with_actor(script, stats, cvars, None)
}

pub fn process_script_with_actor(
    script: &str,
    stats: &mut ResMut<PlayerStats>,
    cvars: &mut ResMut<CVarRegistry>,
    mut actor: Option<&mut Actor>,
) -> Vec<String> {
    let mut output = Vec::new();

    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        // Skip comment lines
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        // Dispatch to command handlers
        let command_output = match tokens[0] {
            "setvar" => cmd_setvar(&tokens, stats, cvars),
            "getvar" => cmd_getvar(&tokens, stats, cvars),
            "listvars" => cmd_listvars(&tokens, stats, cvars),
            "savecvars" => cmd_savecvars(&tokens, stats, cvars),
            "add_gold" => cmd_add_gold(&tokens, stats, cvars),
            "add_stamina" => cmd_add_stamina(&tokens, stats, cvars),
            "quit" => cmd_quit(&tokens, stats, cvars),
            "do_damage" => {
                if let Some(ref mut actor_ref) = actor {
                    cmd_do_damage(&tokens, actor_ref)
                } else {
                    "do_damage can only be used on actors".to_string()
                }
            }
            _ => format!("Unknown command: {}", tokens.join(" ")),
        };

        output.push(command_output);
    }

    output
}
