# Guidance for building the Cauldron: what the sources support

Written 2026-11-02. **Status: findings and proposed rules, not decisions.** Nothing here changes code. Read with the companion note [`2026-11-02-game-editor-best-practices.md`](2026-11-02-game-editor-best-practices.md), which covers the thinner earlier sources.

## What this document is, and what it is not

The owner asked for complete guidance for an editor meant to be enterprise level. **The evidence base cannot support "complete".** What it supports is the *user experience and process* side of an editor, from one book, plus a few findings on undo design and on interfaces for AI agents. Section 7 lists what is not covered (architecture, testing, performance, accessibility, collaboration and more) so nobody mistakes this for the whole job.

Every rule below says where it comes from and how strong that source is. Where I add something the sources do not say, it is marked **inference**.

## 1. Sources and how far to trust them

| Source | What it gives | Strength |
|---|---|---|
| Lightbown, *Designing the User Experience of Game Development Tools*, 2015 (first edition), CRC Press. 164 pages, added to home-still as `lightbown_designing_ux_of_game_dev_tools_2015` | A process (Analysis, Design, Evaluation) and about 200 specific techniques | **Author's experience.** The book cites almost no data of its own; its time-saved figures are a worked hypothetical. A few rules lean on secondary citations (below). |
| Nielsen 1993 (response times), Fitts 1954, Myers 1985 (progress bars), Nielsen on five test users, Miller 1956 and later work on memory, all **as reported by Lightbown** | Numbers behind some rules | **Established findings, second-hand.** I did not read the originals. |
| Ousterhout, *A Philosophy of Software Design*, section 6.7 | One general undo history, small specific actions, grouping decided by high-level code | Read in the home-still corpus. A design argument, not an experiment. |
| Yang et al., SWE-agent, NeurIPS 2024, DOI 10.52202/079017-1601 | Interface design changes agent success; four principles; ablations | **Measured**, one benchmark, 2024 models. |
| Bevy editor prototypes pages | The Bevy project's own scope and open questions | Plans, not results. |

**Edition caution.** This is the 2015 first edition. The 2025 second edition adds material on AI and game tools that is not here.

**How the book was read.** Three reading agents each extracted practices from a third of the book (48, 71 and 83 items), in their own words with page numbers. I read the extractions and checked four claims against the text (the three response-time limits, five test users, the seven-option radio-button rule, and no confirm dialogs on frequent commands); all four matched. I did not read the whole book myself. A few figure labels in the conversion are garbled; the prose is intact.

## 2. Process rules (how to run the project)

1. **Run a loop of Analysis, Design, Evaluation, and repeat.** Observe and find problems with no solutions yet; design within what was found; test with users; go round again in small steps. *Lightbown, ch. 2 (pp. 15 to 26).* **Check:** every UI epic has a recorded observation, a design and a test before it is called done.
2. **Watch people work; do not rely on what they say.** Asking misses what they do automatically; usage counts say *what* but not *why*. *Lightbown pp. 40 to 41.* **Check:** each major workflow has at least one observed session on file.
3. **Ask about goals, not features.** Users will agree to nearly any feature offered. Ask what they are trying to do, then design the feature. Cap the goal list at about a dozen; more means the scope is too wide. *Lightbown pp. 32, 59 to 61.*
4. **Name who the tool is for, and weight effort by hours of use.** Do not design for "everyone", and do not treat one or two experts as representative. Savings are users times hours in the tool. *Lightbown pp. 28 to 32.* **Check:** a written list of user roles exists, with how often each uses the editor.
5. **Measure two things: efficiency and learnability.** Efficiency: stopwatch time for a fixed task with the same assets for every user. Learnability: time for a new user to finish the task on the first try. Record a baseline and repeat each round. *Lightbown pp. 66 to 67.* Do not accept "everyone is faster now" from a lead without numbers (*p. 132*).
6. **Test with about five real users of one role.** Not a mix of roles. The book notes the number is debated. Sessions under an hour, one person runs it and one takes notes, record the screen, do not help when they struggle. *Lightbown pp. 128 to 130.*
7. **Do a heuristic review while waiting for users.** Step through each task flow against Nielsen's ten heuristics; log the issue, the heuristic, and a High, Medium or Low severity. It is a first pass, not a substitute for users. *Lightbown pp. 123 to 126.*
8. **Prototype cheaply before coding when the change is about layout or learnability.** Sketch, paper with a person moving the pieces, or a clickable mock. Go straight to code for efficiency tests, because prototypes cannot reproduce real response time or large data. *Lightbown pp. 117 to 121.*
9. **Staff usability work through the whole project, not in a burst before launch.** *Lightbown pp. 128 to 129.* **Check:** the issue tracker has recurring UX work, not only a pre-release push.
10. **Give the process an owner.** One person drives it, otherwise the whole team does. *Lightbown pp. 25 to 26.*
11. **Count the cost of unlearnable features in time.** Minutes lost per day times users is the argument for learnability work. The book's example numbers are hypothetical. *Lightbown pp. 7 to 10, 51 to 52.*

## 3. Scope rules (what to build)

12. **Order: useful, then usable, then desirable.** *Lightbown p. 3.*
13. **Do a few things really well.** Question the feature list before asking how to build all of it. *Lightbown pp. 34 to 38.* **Check:** a short statement of what the editor deliberately does not do, in the style of the Bevy editor page.
14. **Challenge inherited features, and do not rank an expert-only feature equal to one most people use.** Exception: setup features that could stall other people's work. *Lightbown pp. 35 to 36.*
15. **Count feature interactions.** The book asserts that each new feature adds complexity because it combines with every existing one. *Author's claim, pp. 38, 113.*
16. **Balance developer, stakeholder and user needs.** Favouring any single group breaks the tool. *Lightbown pp. 13 to 14.*

## 4. Interaction rules (how it should behave)

**Speed and feedback**

17. **Response-time limits.** Around 0.1 s feels instant and needs no feedback. Around 1 s is the edge of uninterrupted flow: show a wait cursor. Around 10 s is the edge of attention: show a progress indicator with time remaining. *Nielsen 1993, via Lightbown pp. 93 to 94.* **Check:** every operation that can exceed 1 s has a defined indicator; none exceeds 10 s without a percentage or estimate.
18. **Long work must not lock the editor.** Show progress and let the person keep working. *Lightbown p. 95.* The Cauldron's issue list already has a background-jobs item.
19. **Never be silent.** A tool that gives no sign of life invites a force-quit and lost work. *Lightbown p. 93.*
20. **Do not drown the person in feedback.** Too much gets ignored, including the important part. *Lightbown p. 94.*
21. **Preview before commit, especially for destructive actions.** Show the result on hover or drag before release. *Lightbown pp. 95 to 97.* This is the natural bridge to the `plan` step of the Cauldron's operations.

**Structure and discoverability**

22. **Group commands as the user thinks of them, not as the code is organised.** Use a card sort with real users to find the groups; group by purpose, not by type of effect. *Lightbown pp. 97 to 102.*
23. **Present information in familiar chunks, and follow existing format standards.** Do not reorder a colour format that experienced users read in pairs. *Lightbown pp. 102 to 104.* The book states four, plus or minus two, items of working memory; treat that as a design guide, not a precise number (secondary citation).
24. **Start simple and reveal on demand, but weigh it against extra clicks.** Use it for large tools used all day; skip it for small ones. *Lightbown pp. 112 to 116.*
25. **Support the ramp from beginner to expert.** Visible commands for discovery, context menus for the middle, hotkeys and hideable chrome for experts. Removing any layer breaks someone. *Lightbown pp. 52 to 54.*
26. **Do not assume a new control is seen.** Check by observation. *Lightbown p. 72.* This applies to the Cauldron's shortcut-badge overlays and any new panel.

**Effort per action**

27. **Model each task as look, think, act, and cut the heaviest first: thinking, then looking, then moving.** A few more clicks that need no thought beat fewer clicks that do. *Lightbown pp. 45 to 47, citing Weinschenk and Krug.*
28. **Reduce travel for frequent actions.** Make frequent targets larger and closer (Fitts's law, 1954). Prefer context menus for frequent actions; order menu items by frequency, not alphabet. *Lightbown pp. 104 to 111.*
29. **Hotkeys.** Frequent commands one-handed, near the resting hand; allow remapping; **never override the standard Undo, Cut, Copy, Paste, Save, Open, Close, Select All.** *Lightbown pp. 110 to 111.*
30. **Friction only where it prevents harm.** A confirmation for rare destructive actions, awkward keys for irreversible ones. **No confirmation dialogs on frequent commands; give a reliable undo instead.** *Lightbown p. 112.* This sits directly beside the Cauldron's "every operation can undo" design.

**Correctness and clarity**

31. **Make wrong use impossible where you can.** Bounded sliders, typed drop targets, and designs with no wrong way beat warnings. *Lightbown pp. 83 to 85.*
32. **Place controls where their effect is (natural mapping), and show structure instead of making people calculate.** *Lightbown pp. 85 to 91.*
33. **Match the tool's model to the person's mental model, and use their words.** Name things in user vocabulary, show a live preview for abstract parameters, and expose a familiar structure while hiding the internal data model. *Lightbown pp. 54 to 57.*
34. **Detect likely mistakes and offer the fix.** If the tool can see a texture looks like a normal map but is not flagged, say so. *Lightbown p. 8.*
35. **Remember the person's choices between sessions.** Forgetting view settings is called out as hostile. *Lightbown p. 8.* **Check:** close and reopen; layout, last view and settings return.
36. **Error messages say what happened and how to fix it.** *Lightbown p. 124.*
37. **Reuse standard interaction patterns; do not invent controls.** Follow platform guidelines; if none exist, survey similar controls and copy the common behaviour. A custom widget needs a clear reason. *Lightbown pp. 77 to 79, 141 to 142.*

**Appearance**

38. **One visual language.** The same thing has the same colour and shape in every window. Use standard colour meanings (never red for a non-problem). Prefer a familiar icon over a better-drawn new one. *Lightbown pp. 73 to 75.*
39. **Meet contrast standards, and offer a dark or light choice when context varies.** *Lightbown pp. 76 to 77, with a pointer to W3C contrast rules.* **Check:** automated contrast test across every theme (the Cauldron's skin already carries its own contrast checks, per the blockers note).
40. **Treat polish as a trust signal.** *Lightbown p. 5.*
41. **Do not make documentation the plan.** Docs go stale and experts become a single point of failure; make basic use learnable without them, and keep help, a place to ask, and in-context help. *Lightbown pp. 29 to 30, 124.*

## 5. Rules from the other sources

42. **One general undo history; every change is a small action that knows how to reverse itself; the user interface decides what groups into one undo step.** *Ousterhout, section 6.7.* The Cauldron's operation registry matches the first two. **Open check:** who decides grouping, and does one agent request that changes several things become one undo?
43. **Treat AI agents as a distinct kind of user and design their interface on purpose.** Simple actions, compact actions that do the common thing in one step, short feedback that states the effect, and guardrails that catch mistakes early. Measured effects on SWE-bench Lite (18.0% resolved in the full set-up): no dedicated edit command 10.3%; edit without a syntax check 15.0%; whole file shown instead of a 100-line window 12.7%; iterative search 12.0%. *Yang et al. 2024.* One benchmark; direction credible, sizes not transferable.
44. **Apply Lightbown's methods to the agent as a user (inference).** The book's tools work on any user whose goals and failures can be observed: write the agent's task flows, time them, find where it loops or misuses a tool, and re-measure after each change. Nothing in the sources tests this on an editor.
45. **Keep the editor separate from the running game and talk over a defined protocol; keep GUI-free functions callable without the GUI.** *Bevy editor prototypes pages.* The Cauldron drives the game over the Bevy Remote Protocol already.
46. **Scene and settings data should merge cleanly in version control.** *Bevy editor prototypes pages.*

## 6. How the Cauldron stands (audit questions, not verdicts)

I did not audit the editor against these rules. The table lists what I already know from earlier work in this session and what must be checked. "Known" means I read it earlier; it is a snapshot.

| Rule | What I know | To check |
|---|---|---|
| 42 undo | Operations have `plan`, `commit` and `Undo` in one registry. **Known.** | Grouping into one user-level step; whether view, selection and settings are undoable (the book's example restores selection and view too). |
| 21 preview | `plan` is pure. **Known.** | Whether the person sees a preview before commit, not only the agent. |
| 30 no confirm on frequent actions | A confirm dialog exists for the project-command question. **Known.** | Which frequent actions ask for confirmation. |
| 29 hotkeys | A confirm question answered with a letter key can also act as a command key for other questions (for example `N` on the Map door). **Known defect**, from the risk review. | A full table of keys against the standard-key list. |
| 17 to 20 feedback | The status line reports outcomes. **Known.** Issue list has background-jobs, a status bar and a first-run flow. | Time to feedback for every operation; which exceed 1 s and 10 s. |
| 35 persistence | Not examined. | Close and reopen test. |
| 39 contrast | The skin passes its own contrast checks (blockers note). **Reported.** | Every theme, every text colour. |
| 33 vocabulary | Not examined. | Read all labels and error text for programmer terms. |
| 43 agent interface | About 76 operations each also an MCP tool. **Known.** | Description length, result size, whether results state the effect, guardrails before commit; a fixed task set for measurement. |
| 5 measurement | No usability baseline found in what I read. | Whether any exists. |

## 7. What this does not cover (needs its own research)

The sources say little or nothing about the following, so I will not write rules for them from memory. Each could be a separate research item.

- **Architecture and data model:** partly covered in [`2026-11-02-editor-architecture-and-data.md`](2026-11-02-editor-architecture-and-data.md) (versions, compatibility, merging, layering). Still open: runtime architecture and the extension interface.
- **Testing:** how to test an editor (UI, undo, agent parity), flaky-test policy, the "protected test" rule that blocks 29 issues.
- **Performance budgets** for large scenes and responsiveness targets beyond the three response-time limits.
- **Accessibility** beyond contrast (keyboard-only use, screen readers, motion).
- **Collaboration and version control** workflows, merge conflicts on generated files.
- **Security and trust** (partly covered in the risk review, which is not published).
- **Telemetry and privacy**, since the book suggests usage analytics but not how to do them responsibly.
- **Release, upgrade and support** processes; crash recovery and autosave.
- **Localisation.**
- **Agent-facing design beyond one paper**, including how other engines' editor servers shape their tools.

## 8. Decisions this leaves with the owner

1. Whether to adopt the process rules (section 2), in particular a measurement baseline and a named owner.
2. Whether to commission an audit of the editor against section 6.
3. Which of the section 7 topics to research first.
4. Whether to obtain the 2025 second edition for the AI and tools material.

## Method and limits

- Source text: the home-still conversion of the 2015 book (300,183 characters, 164 pages). An earlier direct conversion attempt failed with 10 unprocessable regions; the library's own conversion completed later and is what was read. Embedding for search failed once for lack of GPU memory, so semantic search over the book may be incomplete; reading was done from the markdown file.
- Extractions are paraphrases made by sub-agents, with page numbers taken from the book's page markers (plus or minus one). Four claims checked against the text; the rest are unverified by me.
- The book's two case-study ROI tables are garbled in the conversion and approximate; I used none of their figures.
- Nothing in the editor was run or audited for this note.
