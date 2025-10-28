# Fallgray

Fallgray is a long-term, hobby project to create an open source, retro-style, single-player fantasy life simulator role playing game. It is set in the fictional fantasy world of Galthea. It draws inspiration from games like Daggerfall and Barony.

It aims to provide a massive open world, both in game and in lore, that is fun to play and easy for players to contribute to. In-game editors and content repositories make contributions trivial for both novice and expert users. Creating mods and even full new "distributions" are considered a core part of the experience of Fallgray.

The game is intended to be easy to play but very difficult to "win." As a fantasy life simulator role-playing game, you play as an average person in the world of Galthea seeking to make a life of your own. You can do this by farming, trading, and going on small quests in the local areas. Or you can pursue the grand quest of trying to uncover a way to stop the destructive cosmic force known as the Maelstrom from unraveling the world. The game uses a mix of hand-crafted content with procedural elements to ensure each playthrough is a new variation of the world with subtle influences apparent from prior playthroughs.

## Status

Currently the project is just getting started!

![](./media/screenshot-2025-10-28-162934.png)

## Tech stack

Rust core using Bevy and TypeScript (Deno) for scripting and tools.

Architecturally, it is by designed to prefer simplicity, modularity, and code maintainability over raw runtime efficiency or unique functionality. Ease of contribution is a priority for the codebase as well as the game content.

## Roadmap

The major phases of development:

#### Base engine

Using Barony as a template, build a block-based, retro-style RPG engine utilizing Bevy.

- Get the basic building blocks for the engine and user interactions in place
- Use placeholder art and assets; don't worry about art style yet

#### Open world basics

Capture some of the feel of Daggerfall's massive world.

- Get city-states, villages, and wilderness in the game
- Continue to use placeholder art and assets

#### Simulation mechanics

With support of a large world, cities, and dungeons, shift focus to game mechanics to make this feel not just like a roguelike RPG but rather like a fantasy life simulator.

- Add Stardew Valley-esque mechanics to the town
- Use placeholder art and assets

#### Playtesting

All the major gameplay and game mechanics should be in place. Begin playtesting to make sure the game is actually fun!

- Iterate on core game mechanics
- Gradually add detail necessary to flesh out systems that feel incomplete

#### Content

Improve the in-game UI / UX and expand it to allow more direct, intuitive editing of the game world.

- Full in-game mod support
- Modding API
- Expanded content and gameplay mechanics

#### Lore and content

Build out the lore and worldbuilding to ensure design coherency and consistency.

- Begin work on higher quality assets for finalized gameplay systems

#### Version 2.0 and beyond...

A long, long-term goal is to rewrite the engine to use a custom, voxel-based rendering and simulation system.

## Contributing

Contributions are very welcome!

## Directory structure

```
bin/                - locally installed binaries
config/             - <placeholder>
resources/          - non-game assets
extern/             - external assets (not created by contributors)
source/             - all source code
    assets/         - <placeholder>
    cmd/            - all binaries
        fallgray/   - the main game
    common/         - source related files used by multiple projects
    crates/         - shared Rust libraries
    modules/        - shared TypeScript libraries
    scripts/        - build-related single-file scripts
    tools/          - larger utilities
```

## FAQ

#### Why Rust?

I wanted a strongly-typed language as it seems best for refactoring effectively as well as coordinating with a large number of engineers. In my own experience, Rust's tooling is straightforward and helps keep more time spent on the code itself than those tools.

#### Why Bevy?

It is has an active community and is well-documented. Ideally less time will be spent reinventing common game engine subsystems by utilizing Bevy. An eventual project goal is to write a custom voxel-based engine (partly, if not mostly, because that would be enjoyable), the higher priority is to ensure the gameplay goals are met. This means custom code should be deprioritized until then.
