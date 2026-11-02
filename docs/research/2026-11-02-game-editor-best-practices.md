# What game editors and agent interfaces teach, measured against the Cauldron

Written 2026-11-02. **Status: findings, not a decision.** Nothing here changes code.

## The question

Is there published or documented guidance on building a game editor that an AI agent drives as well as a person, and where does the Cauldron (the `bevy_grimoire` editor) already follow it or depart from it?

## How thin the evidence is

The home-still corpus holds **no paper on game-editor design**. Searches for editor architecture, undo, live programming and agent-to-software control returned general software-design books and unrelated papers. What I could use:

| Source | Where from | Confidence |
|---|---|---|
| Ousterhout, *A Philosophy of Software Design*, section 6.7, an undo mechanism | home-still corpus (read) | **measured** (read the passage) |
| Yang et al., *SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering*, NeurIPS 2024, DOI 10.52202/079017-1601 (arXiv 2405.15793) | web, full text read in part (first 100,000 of 204,000 characters) | **measured** for the numbers below |
| Bevy editor prototypes design page, and the Bevy "editor questions" document | web, read | **reported** (plans, not results) |
| Search results on Unity, Godot and Unreal MCP servers | web search snippets only | **reported, unverified**; I did not open the vendors' documentation |
| Lightbown, *Designing the User Experience of Game Development Tools*, 2nd ed., 2025, DOI 10.1201/9781003597780 | found by search, **not read** | **unread**; the most relevant book, worth acquiring |

So this note is a small synthesis of three solid sources. It is not a survey of how Unity, Unreal or Godot are built, and I did not read their code.

## What the sources say

### 1. One general history mechanism, many small actions (Ousterhout)

Ousterhout describes student editors that put undo inside the text class, with special handlers for text, selection and cursor. That leaked information between modules and meant every new undoable thing changed the text class. The fix he gives is three separate parts: a general history that stores actions and walks them back and forth, actions that each understand one kind of change, and the policy for grouping actions (his "fences"), owned by high-level code. One user-level undo then restores text, selection and view together.

### 2. Agents are a new kind of user and need their own interface (SWE-agent)

The paper's claim is that interface design changes agent results even with the same model. Its four principles: simple actions, compact actions that do the common thing in one step, short feedback that reports the effect of the last action, and guardrails that catch mistakes early. Ablations on SWE-bench Lite (full setup resolves 18.0%):

| Change | Resolved | Difference |
|---|---|---|
| No dedicated edit command | 10.3% | -7.7 |
| Edit without the syntax check | 15.0% | -3.0 |
| Show the whole file instead of a 100-line window | 12.7% | -5.3 |
| Iterative search instead of summarised search | 12.0% | -6.0 |

These are one benchmark, one task type (fixing code) and 2024 models. The direction is credible; the sizes may not transfer to a game editor.

### 3. The Bevy project's own editor plans (Bevy prototypes, "editor questions")

- Scope: a scene-creation and debugging tool for mixed teams, "Hammer, not Unity". Not an asset or code authoring tool.
- Settings and assets that **merge cleanly in version control** are a stated goal.
- The editor does **not** inspect the running game. It talks to a game over the Bevy Remote Protocol, and reusable dev tools are plugins compiled into the game behind a flag.
- **Undo is an open question** ("one of the hardest aspects"). A community proposal goes beyond scene data to settings and UI state.
- Hot reload with source changes while scenes are open is also an open question.
- The scene format is expected to become `.bsn`; `.ron` is an acceptable stopgap.

### 4. Editor control by agents is arriving in the big engines (reported only)

Search results say Unity has a first-party MCP beta, Unreal 5.8 an experimental first-party MCP server, and Godot has several community ones (one claims 180 tools). I did not verify any of this. If it is true, "an editor that an agent can call" is becoming ordinary, and the question shifts to how well the interface is designed.

## How the Cauldron compares

Based on what I have read of the Cauldron earlier in this work, not a fresh audit.

| Practice | Cauldron today | Reading |
|---|---|---|
| One shared mechanism for undo (Ousterhout) | Every verb is an `Operation` with `plan`, `commit` and `Undo` in one registry | **Matches.** This is the pattern the source recommends. Not checked: whether grouping ("fences") is decided by the UI layer rather than inside operations. |
| Same capability for a person and an agent | A parity test requires a UI entry and an MCP tool for each operation | **Goes beyond** every source here; none describes it. |
| Simple, compact actions with short feedback (SWE-agent) | About 76 operations, each also an MCP tool | **Unknown.** The count is large. Whether results are concise, and whether tool descriptions are simple enough, was not measured. |
| Guardrails before a mistake spreads | `plan` is pure and `commit` writes; a note fence limits agents to their area | **Partly matches.** The fence is cooperative (see the risk review). |
| Merge-friendly data in version control | RON data files, hot reloaded | **Probably matches**, not checked for diff and merge behaviour of generated files. |
| Editor separate from the running game | Editor drives a game over BRP on port 15702 | **Matches** the Bevy plan. |
| Undo beyond scene data | Not examined | **Unknown.** The Bevy proposal treats settings and UI state as undoable too. |

## What the evidence supports, as options for the owner

None of these is a decision.

1. **Measure the agent interface instead of guessing.** SWE-agent's lesson is that interface shape moves results. A small fixed set of editor tasks, run by an agent against the current tool set, would show whether verbosity, tool count or missing guardrails hurt. *Evidence: moderate, from an adjacent domain.*
2. **Check how undo groups work.** Ask whether one agent request that touches several things becomes one undo step. *Evidence: strong as a design principle; the Cauldron's state is unknown.*
3. **Acquire Lightbown's book into home-still** and read it for tool-usability practice. *The most relevant source I found and could not read.*
4. **Do not copy the big engines' tool counts.** The 180-tool figure is an unverified claim and says nothing about quality.

## Method and limits

- Home-still searches: editor architecture, undo and redo, agent control of software, live programming, game-tool usability. Two calls failed once with a gateway error and were repeated.
- The SWE-agent HTML page is about 204,000 characters; I read the first 100,000, which held the design principles and ablations.
- Web search is US-only and its results are summaries; engine claims in section 4 are second-hand.
- I did not read any engine source code or documentation directly.
