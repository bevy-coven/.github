![Bevy Coven: a black raven with spread wings flying across a yellow full moon, over the black rooftops and clock tower of a town under airbrushed pink and purple clouds, with bats and lightning, in pixel art](banner.png)

The parts of a game that only grow with human authoring (levels, set dressing, dialogue, villagers' lives, interfaces, weather) should grow with agents too. Bevy Coven is a coven of Bevy crates that work alone or together. What the coven knows is written into one grimoire, which your game loads. The Cauldron is the editor where you and its agent, the Familiar, write it.

> ⚠️ **Vibe Coded.** Written by AI agents working from a human's direction, covered by tests rather than a line-by-line human review. Read the code before you rely on it for something you care about.

## The family

|Part|Crate|What it does|Latest|Docs|
|---|---|---|---|---|
|**The grimoire**|[bevy_grimoire](https://github.com/bevy-coven/bevy_grimoire)|The knowledge a game loads: a library of pieces and a map, with the grammar learned from your example.|[![crates.io](https://img.shields.io/crates/v/bevy_grimoire.svg)](https://crates.io/crates/bevy_grimoire)|[docs.rs](https://docs.rs/bevy_grimoire) · [book](https://bevy-coven.github.io/crates/bevy_grimoire.html)|
|**The Cauldron**|[bevy_cauldron](https://github.com/bevy-coven/bevy_grimoire/tree/main/crates)|The editor that writes the grimoire: you and the Familiar place pieces, and the learned grammar brews more. It lives in the bevy_grimoire repository.|git only|[book](https://bevy-coven.github.io/crates/bevy_grimoire.html)|
|**World**|[bevy_elements](https://github.com/bevy-coven/bevy_elements)|Sky, weather, wind, clouds, terrain and water, simulated in compute shaders.|[![crates.io](https://img.shields.io/crates/v/bevy_elements.svg)](https://crates.io/crates/bevy_elements)|[docs.rs](https://docs.rs/bevy_elements) · [book](https://bevy-coven.github.io/crates/bevy_elements.html)|
||[bevy_flora](https://github.com/bevy-coven/bevy_flora)|Procedural stylized trees and shrubs that grow over time.|[![crates.io](https://img.shields.io/crates/v/bevy_flora.svg)](https://crates.io/crates/bevy_flora)|[docs.rs](https://docs.rs/bevy_flora) · [book](https://bevy-coven.github.io/crates/bevy_flora.html)|
||[isomesh](https://github.com/bevy-coven/isomesh)|`isomesh` and `bevy_isomesh`: signed distance fields in, meshes out, with caves and carving.|[![crates.io](https://img.shields.io/crates/v/isomesh.svg)](https://crates.io/crates/isomesh) [![crates.io](https://img.shields.io/crates/v/bevy_isomesh.svg)](https://crates.io/crates/bevy_isomesh)|[docs.rs](https://docs.rs/bevy_isomesh) · [book](https://bevy-coven.github.io/crates/isomesh.html)|
||[bevy_lights](https://github.com/bevy-coven/bevy_lights)|Flickering fires and a budgeted pool of real lights over thousands of virtual ones.|[![crates.io](https://img.shields.io/crates/v/bevy_lights.svg)](https://crates.io/crates/bevy_lights)|[docs.rs](https://docs.rs/bevy_lights) · [book](https://bevy-coven.github.io/crates/bevy_lights.html)|
||[bevy_light_grid](https://github.com/bevy-coven/bevy_light_grid)|An illuminance grid that creature AI can read and follow.|git only|[book](https://bevy-coven.github.io/crates/bevy_light_grid.html)|
|**Life**|[bevy_society](https://github.com/bevy-coven/bevy_society)|Villagers with needs, moods, memories, relationships, rumours and factions living daily lives.|[![crates.io](https://img.shields.io/crates/v/bevy_society.svg)](https://crates.io/crates/bevy_society)|[docs.rs](https://docs.rs/bevy_society) · [book](https://bevy-coven.github.io/crates/bevy_society.html)|
||[bevy_stigmergy](https://github.com/bevy-coven/bevy_stigmergy)|Fading marks on a grid that crowds coordinate through, like ant trails.|[![crates.io](https://img.shields.io/crates/v/bevy_stigmergy.svg)](https://crates.io/crates/bevy_stigmergy)|[docs.rs](https://docs.rs/bevy_stigmergy) · [book](https://bevy-coven.github.io/crates/bevy_stigmergy.html)|
||[bevy_orca](https://github.com/bevy-coven/bevy_orca)|Walkers that steer around each other without colliding (ORCA avoidance).|[![crates.io](https://img.shields.io/crates/v/bevy_orca.svg)](https://crates.io/crates/bevy_orca)|[docs.rs](https://docs.rs/bevy_orca) · [book](https://bevy-coven.github.io/crates/bevy_orca.html)|
||[bevy_speech_bubbles](https://github.com/bevy-coven/bevy_speech_bubbles)|Speech balloons, barks, conversation graphs and a skinnable dialog box.|[![crates.io](https://img.shields.io/crates/v/bevy_speech_bubbles.svg)](https://crates.io/crates/bevy_speech_bubbles)|[docs.rs](https://docs.rs/bevy_speech_bubbles) · [book](https://bevy-coven.github.io/crates/bevy_speech_bubbles.html)|
|**Interface**|[bevy_gooey](https://github.com/bevy-coven/bevy_gooey)|Menus, HUD meters, dialogs and tooltips with keyboard and gamepad navigation.|[![crates.io](https://img.shields.io/crates/v/bevy_gooey.svg)](https://crates.io/crates/bevy_gooey)|[docs.rs](https://docs.rs/bevy_gooey) · [book](https://bevy-coven.github.io/crates/bevy_gooey.html)|
|**Spectacle**|[bevy_carnage](https://github.com/bevy-coven/bevy_carnage)|Characters that fracture into chunks, with wounds and blood, when they die.|[![crates.io](https://img.shields.io/crates/v/bevy_carnage.svg)](https://crates.io/crates/bevy_carnage)|[docs.rs](https://docs.rs/bevy_carnage) · [book](https://bevy-coven.github.io/crates/bevy_carnage.html)|
|**Agents and tooling**|[bevy_fairy](https://github.com/bevy-coven/bevy_fairy)|Lets a coding agent read and drive a running Bevy game over MCP.|[![crates.io](https://img.shields.io/crates/v/bevy_fairy.svg)](https://crates.io/crates/bevy_fairy)|[docs.rs](https://docs.rs/bevy_fairy) · [book](https://bevy-coven.github.io/crates/bevy_fairy.html)|

## Start here

- [The Bevy Coven book](https://bevy-coven.github.io/): how the crates fit together, and how to use each one alone or in the Cauldron.
- [What the Coven is and where it is going](https://github.com/bevy-coven/.github/blob/main/VISION.md): the goals, the projects, and the names (Coven, grimoire, Cauldron).
- [How the crates fit together](https://github.com/bevy-coven/.github/blob/main/docs/ARCHITECTURE.md): the layers, the one real dependency edge, and the engine-free crates.
- [How the Coven writes code](https://github.com/bevy-coven/.github/blob/main/docs/STYLE.md): naming, dependencies, layout, Rust, tests and shipping.
- [How every crate is documented](https://github.com/bevy-coven/.github/blob/main/DOCUMENTATION.md).
- [Contributing](https://github.com/bevy-coven/.github/blob/main/CONTRIBUTING.md).
