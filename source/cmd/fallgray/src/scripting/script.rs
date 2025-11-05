use crate::cvars::{CVarRegistry, CVarValue};
use crate::ui::PlayerStats;
use bevy::prelude::*;

// Command handler type for modular command system
type CommandHandler = fn(&[&str], &mut ResMut<PlayerStats>, &mut ResMut<CVarRegistry>) -> String;

/// Handle the setvar command - sets a console variable value
fn cmd_setvar(
    tokens: &[&str],
    _stats: &mut ResMut<PlayerStats>,
    cvars: &mut ResMut<CVarRegistry>,
) -> String {
    if tokens.len() < 3 {
        return "usage: setvar <variable> <value>".to_string();
    }

    let var_name = tokens[1];
    let value_str = tokens[2];

    // Try to parse as f32 (default for now)
    let Ok(value) = value_str.parse::<f32>() else {
        return format!("Invalid value: {}", value_str);
    };

    match cvars.set(var_name, CVarValue::Float(value)) {
        Ok(_) => format!("{} = {}", var_name, value),
        Err(e) => e,
    }
}

/// Handle the getvar command - retrieves a console variable value
fn cmd_getvar(
    tokens: &[&str],
    _stats: &mut ResMut<PlayerStats>,
    cvars: &mut ResMut<CVarRegistry>,
) -> String {
    if tokens.len() < 2 {
        return "usage: getvar <variable>".to_string();
    }

    let var_name = tokens[1];

    match cvars.get(var_name) {
        Some(value) => format!("{}", value),
        None => format!("Variable not found: {}", var_name),
    }
}

/// Handle the listvars command - lists all console variables
fn cmd_listvars(
    _tokens: &[&str],
    _stats: &mut ResMut<PlayerStats>,
    cvars: &mut ResMut<CVarRegistry>,
) -> String {
    let vars = cvars.list();

    if vars.is_empty() {
        return "No variables defined".to_string();
    }

    let mut output = format!("{} variables:", vars.len());
    for (name, value) in vars {
        output.push_str(&format!("\n  {} = {}", name, value));
    }
    output
}

fn cmd_add_gold(
    tokens: &[&str],
    stats: &mut ResMut<PlayerStats>,
    _cvars: &mut ResMut<CVarRegistry>,
) -> String {
    if tokens.len() < 2 {
        return "usage: add_gold <amount>".to_string();
    }

    let Ok(amount) = tokens[1].parse::<i32>() else {
        return format!("Invalid gold amount: {}", tokens[1]);
    };

    stats.gold += amount;
    format!("Added {} gold, new value: {}", amount, stats.gold)
}

fn cmd_add_stamina(
    tokens: &[&str],
    stats: &mut ResMut<PlayerStats>,
    _cvars: &mut ResMut<CVarRegistry>,
) -> String {
    if tokens.len() < 2 {
        return "usage: add_stamina <amount>".to_string();
    }

    let Ok(amount) = tokens[1].parse::<f32>() else {
        return format!("Invalid stamina amount: {}", tokens[1]);
    };

    stats.stamina = (stats.stamina + amount).min(100.0);
    format!("Added {} stamina, new value: {}", amount, stats.stamina)
}

fn cmd_quit(
    _tokens: &[&str],
    _stats: &mut ResMut<PlayerStats>,
    _cvars: &mut ResMut<CVarRegistry>,
) -> String {
    println!("Exiting...");
    std::process::exit(0);
}

pub fn process_script(
    script: &str,
    stats: &mut ResMut<PlayerStats>,
    cvars: &mut ResMut<CVarRegistry>,
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
            "add_gold" => cmd_add_gold(&tokens, stats, cvars),
            "add_stamina" => cmd_add_stamina(&tokens, stats, cvars),
            "quit" => cmd_quit(&tokens, stats, cvars),
            _ => format!("Unknown command: {}", tokens.join(" ")),
        };

        output.push(command_output);
    }

    output
}
