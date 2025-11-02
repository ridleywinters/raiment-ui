use crate::console::ConsoleState;
use crate::texture_loader::load_image_texture;
use crate::ui_styles::EntityCommandsUIExt;
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerStats {
    pub health: f32,  // 0.0 to 100.0
    pub stamina: f32, // 0.0 to 100.0
    pub gold: i32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            stamina: 50.0,
            gold: 0,
        }
    }
}

#[derive(Resource)]
pub struct Toolbar {
    pub active_slot: usize, // 1-9, 0 for 10th slot
}

impl Default for Toolbar {
    fn default() -> Self {
        Self { active_slot: 1 }
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct FatigueBar;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct ToolbarSlot {
    pub slot_index: usize,
}

pub fn startup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize player stats
    commands.insert_resource(PlayerStats::default());
    commands.insert_resource(Toolbar::default());

    let container_style = vec![
        "flex-row-center gap10 p8", //
        "bg-rgba(0.2,0.2,0.2,0.8)",
    ];
    let icon_style = vec!["width-20 height-20"];
    let bar_style = vec![
        "width-200 height-20", //
        "bg-rgba(0.2,0.2,0.2,1.0)",
        "outline-width-1 outline-rgb(0.25,0.25,0.25)",
    ];
    let pico8_red = "bg-rgb(1.0,0.0,0.3)";
    let pico8_green = "bg-rgb(0.0,0.89,0.21)";

    // Status bars at bottom left
    commands
        .spawn_empty()
        .styles(&vec![
            "absolute width-100% height-100% p8",
            "justify-start align-end",
        ])
        .with_children(|parent| {
            // Container for status bars
            parent
                .spawn(Interaction::default())
                .styles(&vec!["flex-col gap2"])
                .with_children(|parent| {
                    parent // Health bar
                        .spawn_empty()
                        .styles(&container_style)
                        .with_children(|parent| {
                            parent
                                .spawn(ImageNode::new(load_image_texture(
                                    &asset_server,
                                    "base/icons/heart.png",
                                )))
                                .styles(&icon_style);
                            parent
                                .spawn_empty()
                                .styles(&bar_style)
                                .with_children(|parent| {
                                    parent
                                        .spawn(HealthBar)
                                        .styles(&vec!["width-100% height-100%", pico8_red]);
                                });
                        });

                    parent // Fatigue bar
                        .spawn_empty()
                        .styles(&container_style)
                        .with_children(|parent| {
                            parent
                                .spawn((ImageNode::new(load_image_texture(
                                    &asset_server,
                                    "base/icons/foot.png",
                                )),))
                                .styles(&icon_style);
                            parent
                                .spawn_empty()
                                .styles(&bar_style)
                                .with_children(|parent| {
                                    parent
                                        .spawn(FatigueBar)
                                        .styles(&vec!["width-100% height-100%", pico8_green]);
                                });
                        });
                });
        });

    // Gold text (keeping at top for now)
    commands
        .spawn_empty()
        .style("width-100% height-100% justify-start align-start p20 absolute")
        .with_children(|parent| {
            parent
                .spawn(GoldText)
                .text("Gold: 0")
                .style("font-size-16 fg-white");
        });

    // Toolbar icons
    let toolbar_icons = [
        "torch",
        "axe",
        "bow",
        "chest",
        "key",
        "map",
        "book",
        "diamond",
        "camp",
        "question",
        "flag_green",
        "bowl",
        "feather",
        "shovel",
        "glove",
        "letter",
        "foot",
        "heart",
        "sword",
    ];

    // Toolbar at the bottom center
    commands
        .spawn_empty()
        .style("width-100% height-100% justify-center align-end p8 absolute")
        .with_children(|parent| {
            // Toolbar container with interaction area and margin
            parent
                .spawn(Interaction::default())
                .style("flex-row gap4 p4")
                .with_children(|parent| {
                    // Create 10 toolbar slots (1-9, then 0 for the 10th slot)
                    for i in 0..10 {
                        // Map visual position to slot number: pos 0->slot 1, pos 1->slot 2, ..., pos 9->slot 0
                        let slot_number = if i == 9 { 0 } else { i + 1 };

                        // Get icon for this slot (wrap if index exceeds array length)
                        let icon_name = toolbar_icons[i % toolbar_icons.len()];
                        let icon_path = format!("base/icons/{}.png", icon_name);
                        let icon_image = load_image_texture(&asset_server, icon_path);

                        parent
                            .spawn((
                                ToolbarSlot {
                                    slot_index: slot_number,
                                },
                                Interaction::default(),
                            ))
                            .styles(&vec![
                                "width-64 height-64 p4 justify-center align-center relative",
                                "bg-rgba(0.2,0.2,0.2,0.8)",
                                "outline-width-2",
                                if slot_number == 1 {
                                    "outline-rgb(1.0,1.0,1.0)"
                                } else {
                                    "outline-rgb(0.4,0.4,0.4)"
                                },
                            ])
                            .with_children(|parent| {
                                parent
                                    .spawn((ImageNode::new(icon_image),))
                                    .style("width-48 height-48");
                                let label_text = if i == 9 { "0" } else { &(i + 1).to_string() };
                                parent
                                    .spawn_empty()
                                    .text(label_text)
                                    .style("font-size-14 fg-white absolute top-2 left-2");
                            });
                    }
                });
        });
}

pub fn update_ui(
    stats: Res<PlayerStats>,
    toolbar: Res<Toolbar>,
    mut health_query: Query<&mut Node, (With<HealthBar>, Without<FatigueBar>)>,
    mut fatigue_query: Query<&mut Node, (With<FatigueBar>, Without<HealthBar>)>,
    mut gold_query: Query<&mut Text, With<GoldText>>,
    mut toolbar_slots: Query<(&ToolbarSlot, &mut Outline)>,
) {
    // Update health bar width
    if let Ok(mut node) = health_query.single_mut() {
        node.width = Val::Percent(stats.health);
    }

    // Update fatigue bar width
    if let Ok(mut node) = fatigue_query.single_mut() {
        node.width = Val::Percent(stats.stamina);
    }

    // Update gold text
    if let Ok(mut text) = gold_query.single_mut() {
        **text = format!("Gold: {}", stats.gold);
    }

    // Update toolbar slot outlines
    for (slot, mut outline) in toolbar_slots.iter_mut() {
        outline.color = if slot.slot_index == toolbar.active_slot {
            Color::WHITE
        } else {
            Color::srgb(0.4, 0.4, 0.4)
        };
    }
}

// Test system to modify stats with number keys (for demonstration)
// Also handles toolbar slot selection (keys 1-9 and 0)
pub fn update_toolbar_input(
    input: Res<ButtonInput<KeyCode>>,
    stats: ResMut<PlayerStats>,
    mut toolbar: ResMut<Toolbar>,
    console_state: Res<ConsoleState>,
) {
    // Don't process toolbar input if console is open
    if console_state.visible {
        return;
    }

    // Toolbar slot selection (1-9, 0 for slot 10)
    if input.just_pressed(KeyCode::Digit1) {
        toolbar.active_slot = 1;
    }
    if input.just_pressed(KeyCode::Digit2) {
        toolbar.active_slot = 2;
    }
    if input.just_pressed(KeyCode::Digit3) {
        toolbar.active_slot = 3;
    }
    if input.just_pressed(KeyCode::Digit4) {
        toolbar.active_slot = 4;
    }
    if input.just_pressed(KeyCode::Digit5) {
        toolbar.active_slot = 5;
    }
    if input.just_pressed(KeyCode::Digit6) {
        toolbar.active_slot = 6;
    }
    if input.just_pressed(KeyCode::Digit7) {
        toolbar.active_slot = 7;
    }
    if input.just_pressed(KeyCode::Digit8) {
        toolbar.active_slot = 8;
    }
    if input.just_pressed(KeyCode::Digit9) {
        toolbar.active_slot = 9;
    }
    if input.just_pressed(KeyCode::Digit0) {
        toolbar.active_slot = 0;
    }
}

pub fn update_toolbar_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut toolbar: ResMut<Toolbar>,
    slot_query: Query<(&Interaction, &ToolbarSlot)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for (interaction, slot) in slot_query.iter() {
        if *interaction == Interaction::Pressed {
            toolbar.active_slot = slot.slot_index;
        }
    }
}
