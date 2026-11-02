# How the Coven writes code

House style for every crate in [bevy-coven](https://github.com/bevy-coven). Where a crate's own `AGENTS.md` or `CLAUDE.md` disagrees, that file wins for its repository and this one gets fixed.

Two habits are borrowed from the documentation guide and apply to code too: **say when you are reasoning rather than reporting**, and **name the measurement** behind any claim about size, speed or coverage.

## Naming

- **`bevy_` prefix, underscore, never a hyphen.** The exception in the family is `isomesh`, whose engine-free crates (`isomesh`, `isomesh-gpu`) carry no prefix because they do not link Bevy; `bevy_isomesh` is the wrapper that does.
- **A package name matches its directory and its `[lib]` name**, so `use bevy_gooey` works with nothing to remember. A binary target is named for the package, not for the verb it runs.
- The words are fixed: **Coven** is the organisation, **grimoire** is the knowledge a game loads, **Cauldron** is the editor, **Familiar** is the agent beside the person in it. A crate that is not the editor does not call itself a grimoire.
- **A crate is named for what it does for a game**, never for the repository it lives in or the product it was extracted from.

## Dependencies

- **A crate in this family may depend on another only when the dependency is the point**, and it says so in a comment beside the line. The one real edge today is `bevy_society` → `{bevy_orca, bevy_stigmergy, bevy_speech_bubbles, bevy_fairy}`; everything else is a leaf. See [`ARCHITECTURE.md`](ARCHITECTURE.md).
- **Engine-free means no engine.** Data, arithmetic, solvers and seeded RNG do not link Bevy, and a crate that makes that promise carries a test that fails if `bevy` appears.
- **`cargo add` before a new dependency.** A crate that grows a dependency grows its consumers' build; the reason goes in the comment, not in a commit message nobody re-reads.
- **One Bevy version, family-wide** (`0.20.0`). A crate naming its own version does so only for `default-features = false`, and it says why.
- **Prefer the family over a third party** for the job a Coven crate already does, and log the reason when you do not.

## Layout

- **One plugin per domain; split by domain, not by kind.** A `systems.rs` that holds every system is a sign the domain was not found.
- **The engine-free half is its own crate**, not a feature flag on an engine-linking one.
- **One integration-test binary per crate** (`autotests = false` plus a single `tests/all.rs` declaring `mod` per file). This is measured, not stylistic: 150 separate Bevy-linking test binaries took a 10-core machine more than an hour to gate; one binary took 108 s to build and 24 s to run.
- **A test file is declared where the test binary can see it**, or it never compiles; a crate guard test refuses an undeclared file.

## Rust

- **`edition = "2024"`, `rust-version` declared.** The family's MSRV is `1.97.1`; a crate with a lower requirement states it and keeps it true in CI.
- **`clippy --all-targets -- -D warnings` is the bar**, not a suggestion.
- **No `unwrap()`/`expect()` on caller data, no panicking index.** Parameters come from callers and from files; a hostile one must produce a refusal or a clamp with a reason, never a panic and never an unbounded loop. A range that a human can type is a range the crate validates.
- **`unsafe` needs a `// SAFETY:` comment that states the invariant**, and the crate's `AGENTS.md` says whether it is allowed at all.
- **Determinism is a feature.** A seeded RNG is seeded once per simulation and threaded; `HashMap` iteration order does not decide an output; a test that can pass or fail by luck is a bug.
- **Delete dead weight.** No compatibility shims, no re-exports kept "just in case", no second way to do a thing that already has a way. A cutover migrates every caller in the same change.

## Errors and messages

- **A refusal says what was asked, why it cannot be done, and what to do instead.** A message that promises a route the code does not have is a defect in the message.
- **A refusal goes where a person looks**, not into a log nobody reads: the UI's problem channel or the toast, and the journal. A dim note is for an answer, not for a refusal.
- **Never silently overwrite.** A save to a derived path uses a no-replace write; a refusal is atomic with the write, and `EXDEV` is a refusal, not a fallback.

## Comments and documents

- **A comment says why, and measures it.** "Measured 2026-10-29: the 12-hour prune frees 0 GB because the mount is `noatime`" is a comment; "clean up stale files" is not.
- **Cite the source of a claim** — a test name, a command, a bench, a paper — the way [`docs/research/`](docs/research/) does.
- **Do not hard-wrap markdown.** One sentence per line.
- A public item has rustdoc; a changed public item's rustdoc changes in the same commit.

## Tests

- **A bug is reproduced before it is fixed**, and the fix carries a test that fails without it.
- **Protected checks are not weakened.** A test, golden file, fixture or scoring harness is not edited to make a change pass; an addition is allowed, a loosened assertion is not.
- **A test that only re-pins wording, a copy, or a mock's echo is not a test.** Test behaviour, boundaries, invariants and refusals.

## Shipping

- **Every visible change updates the README and, if it can be seen, its GIF** — the README's rules are in [`DOCUMENTATION.md`](../DOCUMENTATION.md).
- **The changelog gets a line for every change**, and a visible one gets its clip.
- **A version bump is a release, published locally from a green gate**, never by waiting on CI. Never yank; fix forward.
- **At most one release every six hours** per crate, so crates.io is not hammered into a rate limit.

## House style with no evidence behind it

These are conventions, not findings, and they are here so 14 crates look like one family: the 250-line README cap, the GIF budgets, the plain-words rule, and the "one sentence per line" markdown. They are not measured; do not defend them as if they were.
