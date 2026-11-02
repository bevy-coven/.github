# Architecture and data for the Cauldron: what the sources support

Written 2026-11-02. **Status: findings and proposed rules, not decisions.** Nothing here changes code. This is the second guidance note after [`2026-11-02-game-editor-guidance.md`](2026-11-02-game-editor-guidance.md), which covered user experience and process. Rule numbers here start again at 1 and are independent of that note.

## Scope

"Architecture and data" is wide. This note covers what the sources reached: **how saved data changes over time** (versions, compatibility, migration), **how saved files behave under version control and layering**, **whether real-time collaboration needs special data structures**, and **how the Cauldron's own files and rules compare**. It does not cover the runtime ECS design, the plugin interface, the network protocol or crash recovery; section 7 says why.

## 1. Sources and how far to trust them

| Source | What it gives | Strength |
|---|---|---|
| Kleppmann, *Designing Data-Intensive Applications*, 2017, chapter 4 (Encoding and Evolution). In the home-still corpus | Backward and forward compatibility; schema evolution rules for Protocol Buffers, Thrift and Avro | Read the passages. A widely used practitioner text; not an experiment. |
| Drysdale, *Effective Rust*, 2024, items on semantic versioning and public dependencies. In the corpus | How Cargo treats versions; what a breaking change is; the cost of exposing another crate's types | Read the passages. |
| Shapiro, Preguiça, Baquero, Zawirski, *Conflict-free Replicated Data Types*, 2011, DOI 10.1007/978-3-642-24550-3_29. In the corpus | When replicas can merge without coordination | Read the passages. A peer-reviewed formal result. |
| *Complete CALM: A Coordination Criterion for Specifications*, arXiv 2602.09435. In the corpus | When coordination is unavoidable | Read one passage only. A 2026 preprint. |
| Unity documentation on Smart Merge, and the Unity version-control search results | How a major engine keeps scene files mergeable and what it admits does not work | Documentation page read; the rest is search snippets. |
| NVIDIA's Learn OpenUSD pages (search snippets only) | Layered, non-destructive overrides | **Reported**, not read in full. |
| Bevy editor prototypes pages and the Bevy "editor questions" document (read in the earlier note) | The Bevy project's own open questions on undo and the scene format | Plans, not results. |
| The Cauldron's own documents and code (`docs/guide/data-files.md`, `docs/guide/reload-contract.md`, `crates/bevy_grimoire_core/src/format_version.rs`, `atomic_file.rs`, `ron_surgery.rs`) at `bevy_grimoire` commit `17f7e273` | What the editor does today | **Read** (not run). A snapshot of 2026-11-02. |

**Not obtained.** Godot's text scene format page returned "not found" twice. Unreal's versioning and serialization returned nothing from search. I have no source for either, so I say nothing about them. Two home-still hits that look relevant, the "Dive into Claude Code" paper (arXiv 2604.14228) on extension mechanisms and RAG-MCP (arXiv 2505.03275) on tool selection with many tools, I saw only as search fragments; they are listed for follow-up, not used for rules.

## 2. What the sources say

### 2.1 Saved data must survive change (Kleppmann)

- Two properties matter. **Backward compatibility:** new code reads old data. **Forward compatibility:** old code reads new data. Both are needed whenever different versions of a program meet the same data, which happens as soon as a project is shared between people on different editor versions.
- In the schema-driven formats he discusses, a field added after launch must be optional or carry a default; a field can be removed only if it was optional; and a removed field's identifier must never be reused, because old data may still hold it.
- Changing a field's type can lose information: old code reading a wider number can truncate it.
- When a program reads a record, changes part of it and writes it back, it can silently drop fields it does not know about. Kleppmann says to preserve unknown fields in that case (his Figure 4-7).
- Formats that carry **no version number** force a reader to guess; he recommends a version at the front of each record or file.
- Language-specific serialization usually gives neither kind of compatibility.

### 2.2 Versions are a public promise (Effective Rust)

- Under Cargo's version rules a breaking change needs a new major version, an addition needs a minor, a fix a patch. Version `0.y.z` means anything may change; a change in the leftmost non-zero part signals a break.
- Exposing another crate's types in your public interface ties your versions to theirs: when the dependency makes a breaking release, so must you. This matters to a family of crates that build on each other.

### 2.3 Text formats can be merged, with limits (Unity)

- Unity keeps scenes and prefabs as text so version control can merge them, and its own guidance warns that switching to binary loses that. Its Smart Merge tool understands the structure but handles only scene, prefab and metadata files, passes unresolved conflicts to a fallback tool, and leaves some properties excluded so that both sides changing them is a conflict. Unity itself suggests file locking for teams that cannot tolerate lost work.
- Asset references rely on generated identifiers stored beside each asset. A third-party guide (not authoritative) reports that a conflicted metadata file can lead to a regenerated identifier, which silently breaks references. Treat this as a plausible failure to test for, not a measured one.

### 2.4 Layered overrides (USD, reported)

NVIDIA's documentation describes stacking layers so that a stronger layer overrides a weaker one without editing it, and using separate layers for separate disciplines. A single change that alters many things at once (geometry, materials, hierarchy) is called hard to reason about. Snippets only.

### 2.5 Real-time collaboration needs different data structures (CRDT paper, CALM)

- Replicas can converge **without any coordination** if concurrent operations commute, or if states form a structure with a well-defined merge. The authors say designing for commutativity from the start is cleaner than operational transformation, and that most published decentralized transformation algorithms they cite were shown incorrect.
- "Last writer wins" is simple but loses concurrent updates.
- The CALM paper's example: a replicated counter with a **reset** cannot be made consistent without coordination. Operations that undo or overwrite are the hard ones.
- This is the cost of real-time multi-user editing. It is only relevant if the Cauldron wants that; the Bevy plans instead aim at clean merges in ordinary version control.

### 2.6 The Bevy project's own questions

Undo, the scene source of truth (the planned `.bsn` format versus `.ron`), hot reload when code changes, and where preferences live are all listed as open (see the earlier note). The Cauldron has working answers to some of them, below.

## 3. How the Cauldron stands today

From its own documents and code, as of the snapshot. I read these; I did not run the editor.

- **Everything saved is text.** About twenty kinds of project file, almost all RON, plus an append-only operation log in JSON lines. Per-user files (recent projects, settings) are kept apart from the project.
- **Version numbers exist but are uneven.** The data-files page lists a format version for the map (7), kits (3), rigs (3), project (2) and several at 1, and "none" for the vocabulary, forest and the log. Code comments for the vocabulary and water formats mention refusing another version, which does not match the page's "none" for the vocabulary. **To check.**
- **Policy: refuse, never migrate.** Several formats say in their source that a file of another version "is refused, not migrated". `format_version.rs` is the shared mechanism: a file with no version counts as version 1, and any other number than the build's is refused with the field named. The module says no format has a migration yet.
- **Old files are frozen as test data.** A test folder `old_data` holds files from earlier releases (directories named `core-0.1.0`, `core-0.3.0` and one editor commit) so adding a field with a default keeps them readable.
- **Unknown fields are refused, not preserved.** Eighty uses of `deny_unknown_fields` in the core crate. So an older editor reading a newer file stops with a named error, which matches the "refuse and name it" rule, rather than carrying the unknown field through.
- **Writes are atomic.** Every project-file write goes through one function that writes a temporary file, syncs, renames, and flushes the directory; a test fails if new plain file writes appear. This addresses a reader seeing half a file.
- **Text edits preserve comments.** `ron_surgery.rs` splices only the changed bytes into RON files because people write comments in them. Its header describes two files from an earlier project, so the text may be partly old.
- **Saves are small and repeatable.** A map saves with defaults omitted and stamps left unexpanded; the page reports 41 % smaller files and a second save that is byte-identical, both properties that suit version control.
- **A reload contract.** Every file kind has a class: `Live` (replaced in place), `Respawn` (rebuilt) or `Restart` (needs a restart). My count of the table is 22 Live and 12 Respawn, with 8 rows my quick count could not classify. A test fails when the table is out of date.
- **Layering.** Kit pieces are layered with per-project patches; settings are layered user then project. This is the layered-override idea in miniature.

## 4. Proposed rules

Each says where it comes from. "Check" is a question to answer about the Cauldron, not a finding.

**Data that changes over time**

1. **Every saved format carries a version number, with no exceptions.** *Kleppmann ch. 4; the Cauldron's own `format_version.rs` argues the same.* **Check:** the files listed as "none" (vocabulary, forest, operation log, and any others); and the mismatch between that page and the code comments.
2. **State the compatibility promise in a document.** For example: "this build reads every file written by builds from the last N releases and refuses newer ones". Today it is implied by tests, not stated. *Kleppmann: both directions must be designed for.* **Check:** does a statement exist beyond the "refuse, never migrate" lines in source?
3. **Decide what happens when a change cannot be made with a default.** The Cauldron's policy today ends at "a new version with a migration", and no migration exists. An enterprise user with years of projects needs a written path for the first change that breaks old files: who writes the migration, who tests it against the frozen old files, and what a user sees. *Kleppmann; the Cauldron's own reload page.*
4. **Choose between refusing and preserving unknown fields, and say why.** Refusing protects against a misread file but blocks a team where people run different editor versions. Preserving unknown fields allows mixed versions but can hide real mistakes. *Kleppmann, Figure 4-7 discussion.* The Cauldron has chosen refusal; the owner decision is whether that holds once more than one team uses it.
5. **Never reuse the meaning of a removed field or token.** *Kleppmann, on tag numbers.* **Check:** the vocabulary file's tokens: is a removed token ever reused?
6. **Treat a type change as a breaking change.** Widening or reinterpreting a number or a unit can truncate or misread older data. *Kleppmann.*
7. **Keep the frozen old files and extend them.** The `old_data` folder is the right tool; add one for every released format version. *The Cauldron's own practice; Kleppmann's compatibility argument.*

**Files and version control**

8. **Keep scene-like files as text and keep them merge-friendly.** Stable ordering, omitted defaults, one logical thing per line. *Unity guidance; the Cauldron's own byte-identical resave.* **Check:** are the large files (the map is 134 KB to 227 KB in the page's example) ordered so that two people's edits to different placements merge without conflict?
9. **Give merge a tested story, with a tool or a lock.** Unity needed a merge tool and still recommends locking for teams that cannot lose work. **Check:** whether anyone has tried a real two-person merge of a map file.
10. **Use stable identifiers for things that other things refer to, and test what happens when they collide.** *Unity's reference-identifier failure, reported by a third party.* **Check:** how placements and pieces are named, and whether two branches can mint the same identifier.
11. **Write files atomically, and watch for dotfile temp names.** *Already done in the Cauldron.* Keep the ratchet test.

**Layers and settings**

12. **Prefer layered overrides to editing shared source data.** A project's changes sit on top of a shared kit rather than inside it. *USD, reported; the Cauldron's kit patches.* Keep each override small and about one thing.
13. **Keep per-user state out of project files.** *The Cauldron already does; the Bevy plans list preferences as open.*

**Crates and public interfaces**

14. **Follow semantic versioning strictly for the family's crates, and do not expose another crate's types without accepting its release schedule.** *Effective Rust.* **Check:** whether the editor's git-pinned family crates (ten of them) expose each other's types in public interfaces.

**Collaboration**

15. **Decide the collaboration model before choosing data structures.** If the model is ordinary version-control merges, the rules above are enough. If it is simultaneous editing of one scene by several people, the CRDT result says operations should be designed to commute from the start, and operations that overwrite or reset (undo, "set") need coordination. *Shapiro et al.; Complete CALM.* **Check:** the Cauldron's undo and "set" operations against that criterion if real-time editing ever becomes a goal. I am not recommending it.

## 5. Decisions this leaves with the owner

1. The compatibility promise (rule 2): how many past releases a build must read.
2. The migration policy (rule 3): when "refuse, never migrate" stops being enough.
3. Refuse or preserve unknown fields (rule 4), once teams use mixed versions.
4. The collaboration model (rule 15): merges through version control, or simultaneous editing.
5. Whether to audit the open "Check" items above.

## 6. Observations about the existing design (not defects)

- The refusal rule is consistent and explained in source; the data-files page and the code disagree on whether some formats are versioned. The one place the two should match is the one a stranger reads first.
- The Cauldron's atomic writes, frozen old files, ratchet tests and reload table are the habits the sources recommend. For this topic the Cauldron looks ahead of the sources, and I found no source that it contradicts.
- All of this rests on the `main` branch at the snapshot commit, and on documents that the loop updates automatically.

## 7. What this does not cover

- **Runtime architecture:** how the ECS world, the operations and the viewport relate, and where state lives. The corpus search for entity-component-system returned a single short 2024 paper that I did not read.
- **Extension and plugin interface:** how others add operations or file kinds, and what stays stable. The sources I found do not address it for editors; two home-still papers on agent extension mechanisms and tool selection are worth reading first.
- **Protocol versioning:** the editor's remote-control port and its tools, which also carry data across versions.
- **Large scenes:** streaming, partial loading and memory limits.
- **Crash recovery and autosave.**
- **Godot and Unreal data formats:** not obtained.

## Method and limits

- Home-still searches on schema evolution, ECS serialization, plugin compatibility, CRDTs, semantic versioning and operation-based merging; web searches on Unity, Godot, USD and Unreal; one documentation page read in full (Unity Smart Merge); two Godot page fetches failed.
- The Cauldron was read-only: I opened documents and searched source with `grep`, and counted rows in the reload table by eye and script. Nothing was run, built or changed.
- Counts of `deny_unknown_fields` and reload classes are rough.
- Retrieval passages were read directly for Kleppmann, Effective Rust and the CRDT paper; the other sources are summaries or search snippets as marked.
