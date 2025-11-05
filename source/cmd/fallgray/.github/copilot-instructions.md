# Copilot Instructions

## Project Overview

First-person dungeon crawler built with Bevy 0.17.2 game engine in Rust.

## Architecture

- **ECS Pattern**: Uses Bevy's Entity Component System
- **Module Structure**: Organized by feature (console, scripting, ui, collision, etc.)
- **Asset Loading**: Environment-aware paths via `REPO_ROOT` variable
- **Map System**: YAML-based grid maps with wall/floor/entity definitions

## Key Systems

- **Camera Control**: First-person WASD movement with arrow key rotation, collision detection
- **Console**: Tilde-key developer console with command history and cvar autocomplete
- **CVar System**: Console variables for runtime configuration (weapon animations, etc.)
- **Scripting**: Text-based command processor for items and console commands
- **Billboards**: Sprite rendering for items/NPCs that face the camera
- **Item System**: YAML-defined items with script-based interactions

## Code Conventions

- Follow Rust idioms and Bevy patterns
- Use component queries for system logic
- Keep systems focused and single-purpose
- Organize new features into dedicated modules with `mod.rs` exports
- Keep `mod.rs` exports only with implementations in other files
- Keep individual files concise (ideally < 350 lines)
- Use `cvars` for tunable parameters
- Maintain separation between UI, game logic, and rendering

## UI Framework

- Custom styling system via `ui_styles::EntityCommandsUIExt` (tailwind-like syntax)
- Console UI uses flex layouts with custom styles
- UI elements block game input when interacted

## Asset Paths

- Base assets: `base/textures/`, `base/sprites/`, `base/icons/`, `base/items/`
- Data files: `data/map.yaml`, item definitions in YAML
- Use `load_image_texture` and `load_weapon_texture` helpers

## Testing

- Console commands for debugging (setvar, getvar, listvars, add_gold, add_stamina)
