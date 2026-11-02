# Research

The *because* behind a design: the papers, the measurements, the alternatives that were rejected. Deliberately separate from [`../ARCHITECTURE.md`](../ARCHITECTURE.md) and [`../STYLE.md`](../STYLE.md), which state what we do — a reader who wants the decision, not the argument, only has to read those.

## What belongs here

| Kind | An example of the question it settles |
|---|---|
| **A cited source** | a paper that changes a rule: the README rules in [`DOCUMENTATION.md`](../../DOCUMENTATION.md) each name the study behind them |
| **A measurement** | "150 test binaries took 61 minutes; one took 132 seconds" — and the commands that produced both numbers |
| **A decision record** | the options, what each cost, which was chosen, and what would make us revisit it |
| **A post-mortem** | what went wrong, the evidence, and the rule that prevents it |

A document here that ends without a decision is a note, not research — say so at the top, and say when it will be decided.

## The shape of a research document

1. **The question**, in one sentence.
2. **What is measured and what is reasoned**, kept apart. Numbers name their source: a command, a test, a bench, a citation. A claim that was not measured says so.
3. **The alternatives**, and what each would cost.
4. **The decision**, or an explicit "undecided" with the date it is needed by.
5. **The date and the commit** it was written at, so a later reader knows what it was true of.

## Naming

`<date>-<topic>.md`, e.g. `2026-11-02-engine-free-crate-names.md`. A topic that is revisited gets a new file with a new date that links the old one — the old file is not rewritten, because a record of what was believed is the thing worth keeping.

## Papers already relied on

These are cited by [`DOCUMENTATION.md`](../../DOCUMENTATION.md), and a new claim about how people read or trust a repository should extend that list rather than repeat it:

- Prana et al. 2019, *Categorizing the Content of GitHub README Files* — [10.1007/s10664-018-9660-3](https://doi.org/10.1007/s10664-018-9660-3)
- Venigalla & Chimalakonda 2022, *Readme Content and Project Popularity* — [arXiv 2206.10772](https://arxiv.org/abs/2206.10772)
- Lee 2018, *Ten simple rules for documenting scientific software* — [10.1371/journal.pcbi.1006561](https://doi.org/10.1371/journal.pcbi.1006561)
- Tan, Wagner & Treude 2023, *Detecting outdated code element references in documentation* — [10.1007/s10664-023-10397-6](https://doi.org/10.1007/s10664-023-10397-6)
- Chatlatanagulchai et al. 2026, *Agent READMEs* — [10.1145/3840295](https://doi.org/10.1145/3840295)
- Gloaguen et al. 2026, *Evaluating AGENTS.md* — [arXiv 2602.11988](https://arxiv.org/abs/2602.11988)

## Documents

- [`2026-11-02-python-scripting.md`](2026-11-02-python-scripting.md): can the Coven script its crates in Python (Monty, RustPython, PyO3) without losing performance or determinism? **Shelved**: measured, but no concrete need yet. Harness in [`2026-11-02-python-scripting-spike/`](2026-11-02-python-scripting-spike/).
- [`2026-11-02-game-editor-best-practices.md`](2026-11-02-game-editor-best-practices.md): what the little published guidance on game editors and agent interfaces says, compared with the Cauldron. **Findings only**; the home-still corpus has almost nothing on editors.
- [`2026-11-02-game-editor-guidance.md`](2026-11-02-game-editor-guidance.md): 46 proposed rules for building the editor, from Lightbown's book (read via home-still), Ousterhout and the SWE-agent paper, with source strength per rule and a list of what is **not** covered. Findings, not decisions.
- [`2026-11-02-editor-architecture-and-data.md`](2026-11-02-editor-architecture-and-data.md): how saved data should change over time, merge under version control and layer, and how the Cauldron's own files compare. 15 rules; owner decisions listed. Runtime architecture and plugin interfaces **not covered**.

## Already open, and needing a document here

Two questions the family currently answers by living with an inconsistency. Each deserves a file:

- **Do engine-free crates keep the `bevy_` prefix?** Four crates say yes (`bevy_grimoire_rng`, `_solver`, `_snip`, `bevy_oracle`), `isomesh` says no. Renaming is a breaking change to published crates and to `bevy_society`'s dependency.
- **Which `bevy_devshot` is the real one?** The workspace copy is unpublished and the standalone repository exists; crates.io reserves the name once.
- [`2026-11-02-plugin-interface.md`](2026-11-02-plugin-interface.md): what a plugin interface on top of Bevy's `Plugin` could be, what the Cauldron has today (compile-time family crates, a hardcoded list), and what VS Code and Godot do. 12 rules, 5 owner decisions. **No papers read: the corpus search was down**; web pages only.
