# Why the open `bevy_grimoire` issues are blocked

Written 2026-11-02 against `bevy-coven/bevy_grimoire` at `2b4631d0` and `bevy-coven/.github` at `970f71f`. **Status: findings, not a decision.** The decisions this needs are named at the end and belong to the owner.

## The question

39 `bevy_grimoire` issues are open and 35 carry the `blocked` label. What is each one waiting on, and how many wait on each of our crates?

## The short answer

**Almost nothing is blocked by a crate.** One issue waits on a crate (`bevy_elements`). The other 34 wait on a rule or a service, and 29 of them hang off a single decision.

| Waiting on | Issues | What it is |
|---|---|---|
| **A decision about protected test files** (`chrome::NAME` spellings, issue #210) | **29** | Not a crate. See below. |
| A protected-test question of their own | 3 (#152, #201, #235) | Not a crate. |
| The decision model not being served (`big:8093` is down) | 2 (#112, #113) | A service, not a crate. |
| **A crate: `bevy_elements`** | **1** (#193) | `bevy_elements#55` is open. |
| Not blocked (a tracker, a won't-fix, an infra note, an ongoing series) | 4 (#129, #195, #197, #208) | |
| **Total open** | **39** | 29 + 3 + 2 + 1 = 35 blocked |

An issue counts once, under the cause at the end of its chain. #260 reaches both #210 and #235 and is counted once, under #210.

## By crate

| Crate | Issues it blocks today | Evidence |
|---|---|---|
| `bevy_gooey` | **0** | 21 of the blocked issues list a `bevy_gooey` request under **Needs** (22 distinct requests). **All 22 are closed**, and every `bevy_gooey` issue the editor's issues mention (32) is closed. |
| `bevy_elements` | **1** (#193) | `bevy_elements#55`, "Removing ElementsCamera should undo the camera's dressing", is open. #193 is a bug where the viewport draws no world on Linux (Vulkan) with `bevy_elements` on. |
| `bevy_fairy` | 0 | `bevy_fairy#15` (crop a rectangle out of a screenshot) is open and filed by the editor, but no editor issue waits on it. |
| `bevy_lights`, `bevy_society`, `bevy_flora`, `bevy_carnage`, `bevy_light_grid`, `bevy_orca`, `bevy_stigmergy`, `bevy_speech_bubbles`, `isomesh` | 0 | No open editor issue references them as a blocker. |

Two open requests the editor has made of `bevy_elements` are worth knowing: `#55` (blocks #193) and `#56`, a bug where the scene stays over-exposed for about 20 seconds after the clock jumps from noon to night. Nothing in the editor waits on #56.

**What "Needs" meant, and why it no longer blocks.** Each UI issue says "ready only when each Need is closed as completed **and** the editor's `bevy_gooey` pin is moved to a revision that has it." The Needs are all closed. Moving the pin is a routine first commit of each slice: `main` is pinned to a `bevy_gooey` revision from 2026-11-01, and a loop pass has already moved a pin to v0.31.1 in a commit that is not on `main` yet. So the crate dependency has been satisfied and the pin move is a step of the work, not a blocker.

## The chain that holds 29 issues

```
#210  UI-02 The editor's theme is a gooey skin          <- the root
  |
  +-- #211 UI-03, #251 UI-73, #254 UI-80  (depend on #210 directly)
  |
  +-- #224 UI-30 The workbench shell                    <- depends on #210
        |
        +-- #225 menu bar --> #253 first run, #254 overlays, #257 help
        +-- #228 status bar --> #223 background jobs
        +-- #227, #229, #231, #236 to #247, #255, #258, #259
        |       (#243 also waits on #236)
        +-- #260 UI-99 retire the hand-built widgets    <- waits on 13 open issues (#235 to #247)
```

**#210's acceptance item 1** is "`chrome.rs` declares no colour constants". The editor's colours are read **by name** all over the source and in the test files (`ACCENT` 13 places, `SUGGEST` 12, `DIM` 10, `DANGER` 10 in tests alone). Removing the constants means changing those spellings in protected tests.

The build loop's rule is that a test, golden file or threshold is never weakened to make a change pass, and a change that would "remove, loosen, skip or re-pin an existing assertion" stops that item. So the loop cannot do it, and **by its own rule no pass can grant itself the exception**. The last comment on #210 says what would unblock it: either the same kind of owner exception that was given for the `rigs` re-pin on #202 (rename the `chrome::NAME` spellings in the protected tests without changing any assertion), or a statement that UI-99 carries it.

Slice 1 and slice 2 of #210 are already on `main`: the editor's palette is a `GooeySkin` that passes its own contrast checks, and the 12 colours with no skin field are carried as named tones. Only the re-pin and the clip remain.

## The three others held by the same rule

- **#152** (bug: editor tests red with `--features bevy_elements`). Half 1 is done. Half 2's fixture text lives in protected tests: they write `(sun: (hour: 6.0))`, which is not an `ElementsSettings`.
- **#201** (hot reload H4: runtime state keeps its identity). The remaining slice needs an `assert_ne!` in `an_edit_respawns_only_what_changed` changed, and the owner's 2026-11-01 exception explicitly excludes it.
- **#235** (UI-41: shortcut badges on gooey). The pin is moved and gooey now has what it needs, but the acceptance item is "`badges.rs` is deleted". That file is the editor's whole overlay layout engine (2,527 lines: leader lines, an ink census, ground subtraction), and 13 protected tests with about 40 references pin its design. The last comment offers two readings: replace only the badge chips (needs no re-pin, and is a real slice), or delete the whole module (needs an owner exception and a decision on whether leader lines move into gooey).

## The two held by a service

**#112** (visual gate: calibrated checks on captures) and **#113** (kit labelling as decisions). Both need a served decision model at `big:8093`, which is down; one slice of each is built and the rest ships "advisory" or waits. This is infrastructure the family relies on, but it is not one of the crates in the table above.

## What the 4 unblocked issues are

- **#208** is the tracker for the UI series (UI-00).
- **#129** (E-002) is the ongoing "an operation per editor verb" migration; slices 14 to 20 shipped.
- **#195** is a memory-starvation note about the build hosts.
- **#197** is labelled `wontfix` (the Familiar appears as a character).

## Findings about the process, not the crates

1. **The `blocked` label promises something it can't deliver here.** Every blocked comment says the label "is removed automatically when the blocker closes." For these 32 protected-test cases the blocker is a human decision with no issue to close, so the label stays indefinitely. A separate label such as `needs-owner` would show which issues are waiting on a person and which on a crate.
2. **The queue is deadlocked by design.** 29 of the 39 open issues, and essentially the whole UI series, wait on one answer. Until it comes, the loop can only work the few unblocked items.
3. **Comments said "blocked on #224, which waits on #210" in many places.** That is accurate but hides the root. Anyone reading #255 or #258 sees a dependency on a UI issue and would not guess the actual obstacle is a test-file rule.
4. **No crate is currently the bottleneck.** The earlier worry that `bevy_gooey` was holding the UI work is out of date: it was true until its last fixes landed, and those landed.

## Decisions for the owner

1. **#210:** grant the `chrome::NAME` re-pin exception (spellings only, no assertion changes), or say UI-99 carries it. Answering it unblocks up to 29 (#260 also waits on #235).
2. **#235:** chips only, or the whole overlay.
3. **#152 and #201:** whether their protected-test changes are allowed.
4. **#112 and #113:** when the decision model is next served, if at all.
5. **#193:** whether `bevy_elements#55` is wanted soon; it is the only crate-side blocker.

## Method and limits

- All 192 issues (39 open, 153 closed) were fetched with `gh issue list --state all`, with bodies and comments. Dependencies were read from each issue's `**Depends on**` line and traced to the end of the chain. Crate dependencies were read from `**Needs**` lines and from the latest comments, and the referenced issues in other crates were checked for open or closed with `gh issue view`.
- The root of each chain is from the **latest** comments on the issue; a loop may have changed them since. This note is a snapshot of 2026-11-02.
- I did not read the 153 closed issues, so "no other crate is blocking" means no *open* editor issue references one.
- I did not check whether other family-crate issues are blocked on the editor, which is the opposite direction.
