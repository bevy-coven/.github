# Documentation in the Bevy Coven

Everything that spans more than one repository lives here, in [`bevy-coven/.github`](https://github.com/bevy-coven/.github). A crate's own `docs/` is for its users; this folder is for the people and agents building and maintaining the Coven.

## Where a document goes

| The question it answers | Where it lives |
|---|---|
| What the Coven is, where it is going | [`../VISION.md`](../VISION.md) |
| How the crates fit together, and what may depend on what | [`ARCHITECTURE.md`](ARCHITECTURE.md) |
| How we write Rust, and how a crate is laid out | [`STYLE.md`](STYLE.md) |
| How every crate is documented (README, GIFs, `docs/`) | [`../DOCUMENTATION.md`](../DOCUMENTATION.md) |
| How to contribute, file issues, request a consumer change | [`../CONTRIBUTING.md`](../CONTRIBUTING.md) |
| Research, measurements and decisions behind a design | [`research/`](research/) |
| One crate's API and its user guide | that crate's `docs/guide/`, published to the book |

**One copy, one truth.** The convention set by `DOCUMENTATION.md` applies to these files too: a repository's `AGENTS.md` or `CLAUDE.md` **links** a shared guide instead of copying it. If two files here would say the same thing, delete one and link it.

## Rules these documents follow

- **Do not hard-wrap markdown** (every `AGENTS.md` in the family). One sentence per line, however long.
- **Numbers are measured, and the measurement is named.** A claim about size, speed or coverage says where the number came from — a test, a `cargo` command, a `bench`, a cited paper — the way [`research/`](research/) does.
- **Say when you are reasoning rather than reporting.** `bevy_grimoire/docs/PROJECT.md` is the model: it states which parts are measured and which are judgement.
- **Nothing is published from here by accident.** This folder is not part of the book (`bevy-coven.github.io`). If a page should be readable by someone using the crates, it belongs in the book instead, and this folder links it.

## Adding a document

1. Put it in the right row of the table above.
2. Link it from [`profile/README.md`](../profile/README.md) under **Start here**, or from the document that owns the topic — an unlinked file is invisible, because GitHub renders `profile/README.md` as the org page and nothing else.
3. If it replaces something in a crate, remove the copy and leave a link, in the same commit.

## Research

[`research/`](research/) holds the *because* behind a design: the papers, the measurements, the alternatives that were rejected. It is deliberately separate from `ARCHITECTURE.md` and `STYLE.md`, which state what we do — a reader who wants the decision, not the argument, only has to read those.
