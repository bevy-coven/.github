# The Bevy Coven: what it is and where it is going

> ⚠️ **Vibe Coded.** The Coven's crates are written by AI agents working from a human's direction, covered by tests rather than a line-by-line human review. Read the code before you rely on it for something you care about.

## The one sentence

**Bevy Coven is a family of Bevy crates for the parts of a game that only grow with human authoring — levels, set dressing, weather, villagers, dialogue, interfaces — so that those parts can grow with agents too.**

## The problem

A game developer building a world has two tools that do not talk to each other. The editor is where things get placed; the agent is where the intent lives. Every bridge between them is lossy in the same three ways: **the change has no undo entry, no author, and no validation until a reload.** A person cannot see what the agent did; the agent cannot see what the person did.

The Coven's answer is one vocabulary. What a person does by hand and what an agent does over MCP are the **same named operations**, over one project file, with one undo history between them. That is why every crate here is built to be driven — by a keyboard, by a script, or by an agent — rather than merely configured.

## The names

The vocabulary is deliberately small, and the words mean one thing each:

| Word | What it is |
|---|---|
| **Coven** | The organisation, and the family of crates. The name is collective: the crates are independent, and any one can be used alone. |
| **Grimoire** | The knowledge a game loads — a library of pieces and a map, with the grammar learned from an authored example. It is a book of knowledge, not an editor. |
| **Cauldron** | The editor that writes a grimoire: you and its agent, the **Familiar**, place pieces and let the learned grammar brew more. |

So a game depends on the grimoire; a person authors it in the Cauldron; and every standalone crate here is a page that the Cauldron can write into the grimoire. The editor is `bevy_cauldron`; the runtime is `bevy_grimoire`.

## The goals

These are the commitments the family is held to. Each is checkable; where it is not, it is a wish and does not belong on this list.

1. **Every crate earns its place alone.** `bevy_flora` grows trees whether or not you own a Cauldron. A crate that only works inside the editor is a module of the editor, not a crate.
2. **One vocabulary across the human and the agent.** A verb an agent runs is a verb the editor's keymap dispatches. No second path, no key-stroke synthesising.
3. **Engine-free where the engine is not needed.** Data, arithmetic, solvers and the seeded RNG do not link Bevy. `bevy_grimoire_core` exists precisely so a server or a test can hold the world model without a renderer, and its `tests/engine_free.rs` fails the build if a `bevy` dependency creeps in.
4. **Games are the consumers, and the seam is an issue.** A game that needs something asks for it as an issue labelled `consumer` and `consumer:<game>`; the general form is built here, proved by a test here, and the game is never named in the API. Neither side reads the other's backlog.
5. **Every visible change ships a picture.** A feature is not done until its README line exists and, if it can be seen, its GIF does — because a family of 14 crates is judged by whether its claims can be checked at a glance.
6. **The record is in the repository.** Logs, research, decisions and post-mortems are committed, not kept in a chat or a wiki, so what was believed at a given commit can be recovered later.
7. **One loop per repository.** Automation is bounded and observable: a loop is a dumb driver rerunning one command, one fresh process per pass, with the repository as its only memory. Nobody is asked to trust an agent's memory.

## The projects

| Project | State | Where |
|---|---|---|
| **The Cauldron** | the editor; the UI rebuild on `bevy_gooey` is in progress | [`bevy_grimoire`](https://github.com/bevy-coven/bevy_grimoire) (`crates/bevy_cauldron`) |
| **The grimoire format** | usable; load a library and a map into a running game | [`bevy_grimoire_core`](https://github.com/bevy-coven/bevy_grimoire) |
| **The world** | nine crates shipped, from sky to crowd | `bevy_elements`, `bevy_flora`, `isomesh`, `bevy_lights`, `bevy_light_grid` |
| **Life** | villagers, marks, steering, dialogue | `bevy_society`, `bevy_stigmergy`, `bevy_orca`, `bevy_speech_bubbles` |
| **Interface** | menus, HUD, dialogs, gamepad navigation | `bevy_gooey` |
| **Spectacle** | fracture, wounds, blood | `bevy_carnage` |
| **Agents and tooling** | an agent driving a running game over MCP | `bevy_fairy` |
| **The book** | the user guide, published | [`bevy-coven.github.io`](https://github.com/bevy-coven/bevy-coven.github.io) |

The authoritative per-crate list, with versions and badges, is the family table in [`profile/README.md`](../profile/README.md). Do not copy it here: it goes stale.

## What the Coven is not

- **Not a game engine.** Bevy is the engine, at one pinned version family-wide ([`ARCHITECTURE.md`](docs/ARCHITECTURE.md)).
- **Not a framework that must be adopted whole.** No crate here requires another to be useful, except where the dependency is the point (the Cauldron reads a grimoire).
- **Not stable.** Nothing is called stable before `1.0`; the documentation style guide sets `experimental` and `usable` as the only levels below it.

## How this document is kept true

A goal that stops being true is deleted, not softened. When one changes, the commit says why and links the research or the issue that changed it. This file is descriptive of the Coven as it stands and normative about where it is going; the two are separated by tense, and a goal written in the future tense that has no issue behind it is a wish.
