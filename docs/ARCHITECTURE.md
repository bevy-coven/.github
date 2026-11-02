# How the Bevy Coven fits together

This is the descriptive document: what the crates are, what depends on what, and which rules the family already holds itself to. It is measured against the repositories, not aspirational — where a rule is not yet true, it says so.

## The layers

Read bottom to top. A crate may depend on anything **below** it and nothing **above** it.

```
                    ┌──────────────────────────────┐
  authoring         │  bevy_cauldron (the editor)  │  reads and writes a grimoire
                    │  bevy_cauldron_mcp           │  the same verbs, over MCP
                    └───────────────┬──────────────┘
                                    │
                    ┌───────────────▼──────────────┐
  the book          │  bevy_grimoire   (runtime)   │  a game loads a library + a map
                    │  bevy_grimoire_core          │  the world model, engine-free
                    └───────────────┬──────────────┘
                                    │  (core is optional to a game)
  ┌────────────┬────────────┬───────▼──────┬─────────────┬──────────────┐
  │ World      │ Life       │ Interface    │ Spectacle   │ Agents       │
  │ bevy_      │ bevy_      │ bevy_gooey   │ bevy_       │ bevy_fairy   │
  │ elements   │ society    │              │ carnage     │ bevy_devshot │
  │ bevy_flora │ bevy_      │              │             │              │
  │ isomesh    │ stigmergy  │              │             │              │
  │ bevy_lights│ bevy_orca  │              │             │              │
  │ bevy_light_│ bevy_speech│              │             │              │
  │ grid       │ _bubbles   │              │             │              │
  └────────────┴────────────┴──────────────┴─────────────┴──────────────┘
                                    │
                    ┌───────────────▼──────────────┐
  primitives        │  bevy_grimoire_core (data)   │  constraint IR, solvers, WFC
                    │  bevy_grimoire_rng           │  one seeded stream per simulation
                    │  bevy_grimoire_solver        │  SAT, deterministic
                    │  bevy_grimoire_anim          │  clip-weight blending
                    │  bevy_grimoire_snip          │  a rectangle out of a PNG
                    │  bevy_oracle                 │  typed questions to a decision model
                    └──────────────────────────────┘
```

## The dependency rule, and the one real edge

The family is deliberately flat. **Measured 2026-11-02, across every `Cargo.toml` in the org**, there is exactly one crate-to-crate dependency between family repositories:

| From | To | How | Why |
|---|---|---|---|
| `bevy_society` | `bevy_orca` | crates.io `0.2`, optional (`steering`) | walkers steer around each other |
| `bevy_society` | `bevy_stigmergy` | crates.io `0.2`, optional (`fields`) | marks on a grid that crowds follow |
| `bevy_society` | `bevy_speech_bubbles` | crates.io `0.2`, optional (`dialogue`) | barks and conversations |
| `bevy_society` | `bevy_fairy` | **git rev pin**, optional | an agent reads a running society |

Everything else in the family depends on **no other family crate**: `bevy_elements`, `bevy_flora`, `bevy_lights`, `bevy_light_grid`, `bevy_gooey`, `bevy_carnage`, `isomesh`, `bevy_devshot`, `bevy_orca`, `bevy_stigmergy` and `bevy_speech_bubbles` are all leaves. That is the property that makes "use one crate alone" real rather than a slogan.

Inside the `bevy_grimoire` workspace the edges are: `bevy_grimoire_core` → `bevy_grimoire_rng`, `bevy_grimoire_solver`; `bevy_cauldron` → `bevy_grimoire_core`, `bevy_grimoire_anim`, `bevy_grimoire_snip`, `bevy_oracle`; `bevy_cauldron_mcp` → `bevy_cauldron`. Each of those is a **path** dependency, so an editor-only change never touches a game's build.

> **Known exception, to be fixed:** `bevy_society` pins `bevy_fairy` by git revision at `https://github.com/Ladvien/bevy_fairy`, the personal account, while the four crates beside it resolve from crates.io. A git pin also means `bevy_society` cannot be published from a registry-only consumer without a `[patch]`. Both are tracked, not designed.

## Engine-free crates

A crate that does not need Bevy must not link it, and one of them enforces it:

| Crate | Links Bevy? | Enforcement |
|---|---|---|
| `bevy_grimoire_core` | **no** | `crates/bevy_grimoire_core/tests/engine_free.rs` fails the build on any `bevy*` dependency outside an explicit allow-list |
| `bevy_grimoire_rng` | no | none (no test) |
| `bevy_grimoire_solver` | no | none (no test) |
| `bevy_grimoire_snip` | no | none (no test) |
| `bevy_oracle` | no | none (no test) |
| `isomesh` | no | `isomesh` and `isomesh-gpu` are engine-free; `bevy_isomesh` is the wrapper |
| everything else | yes | — |

**The open question this exposes:** four crates carry the `bevy_` prefix without depending on Bevy, which contradicts the reading that the prefix means "this needs the engine". `isomesh` resolves the same contradiction the other way — its engine-free core is `isomesh` and its wrapper is `bevy_isomesh`. Renaming the four is a breaking rename of published crates (`bevy_grimoire_rng` 0.1.0 and `bevy_grimoire_solver` 0.1.1 are on crates.io, and `bevy_society` depends on `bevy_grimoire_rng` from it), so the family currently lives with the inconsistency rather than pay for it. `STYLE.md` states the rule as it is applied today.

## Shared constants

These are facts every crate is currently expected to match. They belong in one place so a check can be written against them.

| Thing | Value | Notes |
|---|---|---|
| Bevy | `0.20.0` | pinned workspace-wide; a crate that needs `default-features = false` names the version itself |
| Rust edition | `2024` | **except `bevy_fairy`, still on `2021`** |
| MSRV | `1.97.1` | most crates; `isomesh` declares `1.89` for its engine-free core |
| Licence | `MIT OR Apache-2.0` | every crate, and `CONTRIBUTING.md` makes contributions dual-licensed by default |
| `repository` | `https://github.com/bevy-coven/<repo>` | **except `bevy_devshot`'s standalone copy, which names the personal account** |

## Duplicated crate names

`bevy_devshot` exists twice: `crates/bevy_devshot` inside the `bevy_grimoire` workspace (`publish = false`) and the standalone [`bevy-coven/bevy_devshot`](https://github.com/bevy-coven/bevy_devshot) repo. crates.io reserves names globally and the workspace copy is unpublished, so whichever is published first owns the name. This is a decision waiting to be made, not a settled fact — do not add a second copy of anything else in the meantime.

## How a change crosses a boundary

The seam between the Coven and a game is an **issue**, not a patch:

1. The game files an issue here labelled `consumer`, `consumer:<game>`, and `bug` or `enhancement`, saying what it needs, the current behaviour, the proposed API, edge cases, and an acceptance test the crate can run alone.
2. The crate builds the **general form** — the game is the example, never a name in the API — and proves it with a test in its own repository.
3. The game takes it when the issue closes as completed and its checkout has the fix; until then it carries `Needs: <repo>#<n>` on its own item.
4. The loop's procedure for this is its own command ("the two-way street"), and each crate is edited **in its own repository**, never vendored or patched from a consumer.

## What this document is not

It is not the API of any crate (that is each crate's `docs/guide/` and rustdoc), and it is not the reason a choice was made — the arguments and measurements are in [`docs/research/`](docs/research/). When this file and a crate's `Cargo.toml` disagree, the `Cargo.toml` is right and this file is a bug.
