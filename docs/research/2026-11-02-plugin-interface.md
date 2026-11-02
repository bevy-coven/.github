# A plugin interface for the Coven and the Cauldron: what the sources support

Written 2026-11-02. **Status: findings and proposed rules, not decisions.** Nothing here changes code. It follows [`2026-11-02-editor-architecture-and-data.md`](2026-11-02-editor-architecture-and-data.md), whose section 7 listed the plugin interface as not covered. Rule numbers start at 1 and are independent of the other notes.

## The goal, in the owner's words

Other people must be able to add their own plugins to the Bevy Coven ecosystem and to the Cauldron, with something that sits on top of what Bevy already provides and makes adding one easy. The Coven's own crates should use the same route wherever possible, so the route is tested by the people who build it.

## Scope and a warning about sources

This note covers: what a "plugin" can mean here, what the Cauldron has today, and what other editors do. It does not cover the Python scripting question (see [`2026-11-02-python-scripting.md`](2026-11-02-python-scripting.md)) or any design for the Coven's ecosystem outside the Cauldron.

**The home-still corpus could not be searched.** Four search calls failed with "gpu busy" (4,560 MB free, 5,000 MB needed to load the embedding model; a vLLM process held about 15 GB on `big`). So **no paper was read for this note.** The sources below are vendor documentation pages read through a fetch tool that summarises them, one search for Bevy plugin crates, and the Cauldron's source. They show what other editors *do*; they are not studies of whether it works. A corpus pass (for example on API evolution, extension security, and plugin ecosystems) is still owed.

## 1. Sources and how far to trust them

| Source | What it gives | Strength |
|---|---|---|
| Bevy `Plugin` trait docs (docs.rs) | The trait's methods and uniqueness rule | **Read** (a fetch-tool summary of the page; the last 5,000 characters were not read) |
| VS Code, activation events page | Extensions declare in a manifest when they load; activation is lazy; `activate`/`deactivate` | **Read** (summary) |
| VS Code, extension host page | Extensions run in a separate host; the stated aim is that they do not slow startup or the UI. The page says nothing about security isolation | **Read** (summary) |
| VS Code, Workspace Trust page | Extensions declare whether they support untrusted folders; the page says trust "can't prevent a malicious extension from executing code" | **Read** (summary) |
| Godot, "Making plugins" page | A folder plus a `plugin.cfg`; the user enables it in project settings; it can add nodes, docks, autoloads | **Read** (summary) |
| One web search on Bevy dynamic loading and mods | Bevy's built-in dynamic plugin loader is described as deprecated and unsound for modding; Wasvy and `bevy_wasm` are experimental or stalled | **Reported** from search snippets. I did not open the crates' repositories, so their current status is unconfirmed |
| The Cauldron's source and docs at `bevy_grimoire` commit `17f7e273` | What extension points exist today | **Read** (not run) |

**Not obtained.** The WebAssembly component model page (my guessed address did not resolve; I did not retry). Unreal and Unity plugin systems. Anything on supply-chain attacks against plugin registries. I say nothing about these.

## 2. What "plugin" can mean here

Three different things share the word, and the answers differ:

1. **A Bevy plugin**: Rust code compiled into the program. Bevy's `Plugin` trait has `build`, plus `ready`, `finish`, `cleanup`, a `name`, and a uniqueness rule (a plugin of the same name can be added only once unless it says otherwise). This is the "sits on top of" layer.
2. **An editor extension**: something that adds to the Cauldron's own surface: an operation, a panel, a key, a data file kind, a guided-checkpoint.
3. **A game-side extension**: something a game's author adds to the data a project holds and the game reads.

Meaning 1 is compile-time: the plugin's author and the build's author must be the same person or must share a build. The Bevy page describing dynamic loading as deprecated (reported, not confirmed) points the same way. So a third party cannot hand a user a binary file and have it load, without the work in section 5.

## 3. What the Cauldron has today

All read from source; nothing was run.

- **Family crates are compile-time cargo features with a hardcoded list.** `family_host.rs` has `KNOWN`, a constant list of nine crate names (`bevy_carnage` … `bevy_stigmergy`), and `built()` has one `cfg!(feature = …)` branch per name. Its doc says "Add a name here in the commit that adds the cargo feature." A project's `family.ron` lists the crates it uses; naming one this build lacks is reported to the author, not ignored. Adding a crate is a change to the Cauldron's own source. **A third party cannot add one.**
- **The editor's plugin graph is a hand-ordered list.** `harness::add_editor_plugins` adds the Cauldron's own plugins, nested because Bevy's `add_plugins` tuples stop at 15. The comments in it record that order matters (the surface plugin first).
- **Operations are a trait with a registry.** `ops.rs` has `pub trait Operation` and `pub fn registry() -> Vec<Box<dyn Operation>>`, with `find` and `search`. There are about 76 operations, shared by the editor's keys and the agent's tools. **I did not read how `registry()` is built**, so whether an outside crate could add an operation is unknown. A check item below.
- **Settings for a hosted crate are files**: `family/<crate>.ron`, reloaded when they change, read as the crate's own type where the editor has that type, otherwise held to being valid RON. The editor "holds no schema of its own: the type is the crate's".
- **Checkpoints, keys and reload kinds are registered by name in the Cauldron's own code** (the guided-checkpoint vocabulary in `guided.rs`, `keys::BINDINGS`, `reload::KINDS`). I read their descriptions in the crate's notes, not the registration code.
- **A project can ask the editor to run a command** (`agent.ron`, `play.ron`). That is a plugin-like route with no trust model today; the review in the owner's risk list covers it, and a trust prompt exists on a local, unpushed branch. Any plugin route must not repeat that gap.

## 4. What other editors do

Stated as the pages state it, and no further.

- **VS Code**: a manifest names *when* an extension loads (activation events) and *what* it adds. It loads lazily and runs in a separate host. The host page's aim is that extensions do not degrade the editor; it does not claim security isolation. Workspace Trust is a second layer: an extension declares whether it supports an untrusted folder, and the user can override the declaration. The trust page itself says this cannot stop a malicious extension.
- **Godot**: a plugin is a folder with a small manifest, enabled by the user per project. Docks, nodes and autoloads are the extension surface. Its main script runs inside the editor's process.

What these share: a **manifest** that says what the plugin is; **lazy or explicit enabling** by the user; a **small named set of things a plugin can add**; and, in VS Code's case, a **published statement of what the host does not guarantee**.

## 5. Proposed rules

Each is a statement for the owner to accept or reject, not a decision. Strength: **Stated** = a source says so; **Inferred** = follows from the sources plus the Cauldron's code; **Open** = nothing read settles it.

1. **A plugin is a Bevy `Plugin` first.** The extension route must take a value that implements Bevy's `Plugin`, not invent a parallel trait. A person who knows Bevy should recognise it. *(Inferred from the goal.)*
2. **One registration path, used by the Coven's own crates.** Today's `KNOWN` list plus a `cfg!` branch is a path only the owner can use. The owner's wish is to use the same route as outsiders. Two paths (one for family crates, one for third parties) would be the double path the owner's rules forbid. *(Inferred.)*
3. **A manifest declares what a plugin adds.** Name, version, the Cauldron versions it works with, and a list of what it contributes (operations, panels, data file kinds, keys, checkpoints). VS Code and Godot both do this. *(Stated for both editors.)*
4. **A closed, named list of extension points.** The plugin can add only what the list names. Each point is documented once. A point that is not on the list is not available, and asking for one is a refusal with a message. *(Inferred from Godot's and VS Code's shapes.)*
5. **A plugin's data is a file kind with a version, like the Cauldron's own.** It uses the same refuse-never-migrate rule, the same atomic writes, and a row in the reload table. Otherwise a plugin is the one part of a project that breaks the rules the rest keeps. See rules 2 and 4 of the architecture note. *(Inferred.)*
6. **Compatibility is declared and checked at load.** A plugin that was built for a Cauldron version the running build does not accept is refused with its name and both versions. Rust has no stable binary interface, so for compile-time plugins this is a Cargo version question; Effective Rust (read in the earlier note) covers why exposing another crate's types in the public interface makes every upgrade a breaking change. *(Inferred; the Bevy dynamic-loading status is reported.)*
7. **A plugin cannot act as the person.** Whatever the plugin adds as an operation is subject to the same fence an agent is: the person's trust grant is never an operation, and an operation a plugin registers cannot be called "trust". *(Inferred from the Cauldron's own trust design in `trusted.rs`.)*
8. **State what a plugin is allowed to do, and what the host cannot stop.** VS Code publishes both. A compile-time Rust plugin runs with the program's full rights; nothing in Bevy's `Plugin` limits it. The documentation must say so, so users do not assume a sandbox. *(Stated by VS Code's trust page for its own case; Inferred for Rust.)*
9. **The user enables a plugin, per project.** The existing `family.ron` already does this for family crates. A plugin the project does not list does not run. *(Stated for Godot; Inferred for the Cauldron.)*
10. **A plugin's failure is shown where the author looks.** The Cauldron already treats a listed-but-missing crate as a message, not a silent skip. The same holds for a plugin that fails to start. *(Inferred from `family_host.rs`.)*
11. **Plugins ship with a test the host runs.** The Cauldron's existing parity tests (every bridge method is registered; every guide checkpoint runs) are the pattern: a plugin's manifest is checked against what it registers. *(Inferred from the Cauldron's tests as described in its crate notes.)*
12. **Dogfood by moving, not by copying.** The first proof that the interface is enough is moving one existing family crate onto it (the owner chooses which), with the old route deleted in the same change. *(Inferred from the owner's rules.)*

## 6. Check items

These are questions to audit, not findings.

- Can an outside crate add an `Operation` today, or does `registry()` list them by name inside the Cauldron? (Not read.)
- Which of `keys::BINDINGS`, `reload::KINDS` and the checkpoint vocabulary are static lists, and which accept a registration? (Not read.)
- Does `family.ron` have a version and a manifest of what each family crate adds, or only a list of names? (It has `version` and `crates`; nothing else, from the code read.)
- Which of the nine family crates would be the smallest one to move first?
- Does the MCP server's tool list update when a plugin adds an operation, or is it generated from the Cauldron's own list? The docs say the tools page is generated and a test fails on drift. How would that work with plugins?
- Where does a plugin's own panel sit in the UI's keyboard model (the keys census in `keys.rs`)? A plugin adding a key can collide with a built-in.

## 7. Decisions for the owner

1. **Compile-time only, or also loadable at run time?** Compile-time (a Bevy plugin added in a build) is the simple path and matches rules 1 and 2. Run-time loading (for users who do not build the editor) needs a different mechanism, such as a sandboxed module; that would be a second path and a much larger project. The reported state of Bevy's own loader suggests care.
2. **Who builds the Cauldron?** If users install a release binary, third-party compile-time plugins cannot reach them. If users build their own editor (as a game's author builds a game), they can. This changes who the "others" are.
3. **Which extension points first?** The smallest set that lets one family crate move across. Suggest asking which crate that is.
4. **Naming and registry.** Are plugins discovered by listing in `family.ron`, by a directory, or by a public registry? A registry brings supply-chain risk this note did not research.
5. **Trust.** Is a plugin's code trusted by being in the build, or does the project need to say which plugins it allows? Rule 9 assumes the latter.

## 8. Not covered, and why

- **Run-time loading and sandboxing.** The component-model source was not obtained, and the Bevy loader's status is from a search snippet.
- **Security of a plugin registry.** No source read.
- **How Unity, Unreal, Blender or Bevy's own editor prototypes expose plugins.** Not searched in this pass; the corpus was down.
- **The academic literature on plugin architectures and API evolution.** The corpus search failed; this is the main gap.
- **Python or script plugins.** See the scripting note.

## 9. Method and limits

Written by reading the Cauldron's source and docs at `bevy_grimoire` commit `17f7e273` without running it, and by fetching five vendor pages and one search. Fetch-tool summaries may omit detail; the Bevy page's tail was not read. The home-still corpus was unavailable, so no paper supports any rule; every rule is labelled *Inferred* or *Stated by a vendor page*, not studied. Next step when the GPU is free: search the corpus for plugin architecture, API evolution and extension security, and revise this note in a new dated file rather than rewriting this one.
