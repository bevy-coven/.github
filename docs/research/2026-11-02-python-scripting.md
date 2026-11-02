# Python as the Coven's scripting language

Written 2026-11-02, against `bevy-coven/.github` at `ccf4081`. **Status: shelved, not rejected.** No concrete need for Python scripting exists today (see "Why this is shelved"), so nothing was decided and nothing was built. This note records what was learned so it can be picked up when a need appears. The harness that produced every number is in [`2026-11-02-python-scripting-spike/`](2026-11-02-python-scripting-spike/).

## The question

Can the Coven script its crates in Python (the language only, no third-party packages), with built-in Python access to every crate, without hurting performance or the family's rules (deterministic, sandboxed, one path, leaf crates stay leaves)?

## What was searched, and what it found

**Measured: nothing.** No benchmark was run for this note. Every number below is quoted from a source, and a claim that is my reasoning says so.

- **home-still corpus** (`distill_search`, three queries: embedding Python in a game engine, sandboxing Python, PyO3/FFI overhead). **Nothing relevant.** The hits were general books (Game Engine Architecture, A Philosophy of Software Design, Rust for Rustaceans) and unrelated papers. Game Engine Architecture has a scripting-language chapter that surfaced; it was not read. The corpus has no source on embedding Python, so no DOI is cited.
- **Web**: the Monty docs and crate page, the RustPython README, the PyO3 guide, the CPython 3.14 C-API docs, and the `bevy_mod_scripting` docs.

## The options

| Option | What it is | Fits the Coven? |
|---|---|---|
| **Monty** (`monty` crate, Pydantic) | A Python interpreter written in Rust: no CPython, no FFI, no C dependencies. 1.4.0 on docs.rs, dated 2026-10-26; the docs call 1.0.0 "ready for production use". Targets CPython 3.14 behaviour for the subset it implements. | **Best fit on paper.** Reasons below. |
| **RustPython** | A Python interpreter in Rust. Its README says "not totally production-ready" and "a fairly young project". WASI build (`wasm32-wasip1`) is documented. | Weaker: no documented resource limits, no performance figures found. |
| **CPython via PyO3** | The real interpreter, linked into the game. Full stdlib, full speed. PyO3 supports free-threaded builds since 0.23. CPython has had per-interpreter GILs since 3.12; whether PyO3 supports sub-interpreters was **not confirmed** (the issue thread found is older). | Weak: no per-call limits, hard to sandbox, a C dependency in every game, no `wasm32-unknown-unknown`. |
| **`bevy_mod_scripting`** | The existing Bevy scripting layer. Lua variants and Rhai; Rune on hold. **No Python.** No WASM support. | Not a route to Python. Its reflection-based binding layer is the model to compare. |

## Why Monty fits, and where it does not

Sourced from the Monty docs (landing page, limitations, resource limits) and the `monty` crate page.

**For:**
- **Sandbox by construction.** "Filesystem, environment variables and network do not exist inside the sandbox." The host is reached only through functions you pass in.
- **Host calls are a pause, not a callback.** Execution stops at an external function call, the host does the work and resumes it (`MontyRun::start` returns a `RunProgress`). That maps onto the pattern `bevy_society` already uses: scripts read a frozen snapshot and queue effects that one function applies.
- **Limits exist:** memory, execution time (per feed and per host round trip), recursion depth (default 1000), and host round trips (default 1000).
- **Snapshots:** the whole sandbox state can be dumped to bytes at a host call. Useful for save games and for the Cauldron's undo log.
- **Pure Rust, no C.** It would build like every other Coven crate.

**Against, or not yet known:**
- **It is a subset of Python.** Rejected at parse time: class inheritance, `@property`/`@classmethod`/`@staticmethod`, generators (`yield`), `del`, `async with`/`async for`. Missing: user-defined exception classes and the `enum`, `contextlib`, `io`, `struct`, `operator`, `string` modules. Dataclasses take only `eq=` and `frozen=`. Scripts that are mostly functions, dicts and simple classes are fine; idiomatic Python class hierarchies are not.
- **Determinism is not documented.** The resource-limits page does not address it and says time limits "may overshoot", so where one fires can vary between runs. `random`, `time`, `datetime` and `os` are present in part. The Coven's rule is that a seeded run reproduces, so those would have to be removed or replaced with the seeded stream (`bevy_grimoire_rng`). **Rhai has an operation-count limit** (`bevy_society`'s `script.rs` sets 10,000 operations, 16 call levels and size caps, lines 41-45), which is deterministic; Monty's documented limits are time and memory, which are not. The spike below confirmed Monty has no instruction-count limit.
- **A crash takes the host down.** The crate page says an abort (a stack overflow, say) kills the process, and recommends `monty-pool` (worker subprocesses) for untrusted code. Memory limits need `monty-alloc` as the program's global allocator.
- **Snapshots are not authenticated.** A corrupt one "may panic, abort, hang, or produce incorrect results". A saved game that loads a snapshot is loading untrusted bytes unless it is checked.
- **WASM is not mentioned in the docs I fetched.** The spike shows it compiles for `wasm32-unknown-unknown` (compile check only).
- **Its API may still move.** 1.4.0 is three weeks old as of this note.

## How "Python access to every crate" could be built (reasoning, not tested)

- **Where the bindings live.** A crate that depends on a scripting crate stops being a leaf, which `ARCHITECTURE.md` treats as the property that makes "use one crate alone" real. So the bindings belong in a layer above the family, in the same place the Cauldron already pins each crate as an optional feature, not inside each crate.
- **Data first, through serde.** Every family crate already has a serde settings or data type, and the Cauldron already edits those. A single bridge between Monty's value type (`MontyObject`) and serde would expose all of them as plain dicts with no per-crate code, and the editor's "one schema, no second one" rule holds.
- **Verbs by hand.** Calls such as "plant a rumour" or "grow this tree" need a registered host function per verb. A thin trait or macro per crate keeps that small.
- **One path.** `bevy_society` scripts are Rhai today. Python beside Rhai is two script paths, which the family rules forbid. Either Python replaces Rhai in `bevy_society` (a breaking change to a published crate), or Python is editor-side only. That is a decision for you, not one to assume.

## Alternatives, and what each would cost

- **Stay on Rhai** and add the same serde bridge. Cheapest, deterministic, but not Python.
- **Python editor-side only**, out of process, driving the Cauldron's existing operations through MCP. No embedding risk at all; no Python inside a shipped game.
- **Monty embedded in the game** at trigger rate, behind a budget. The most capable option, and the one with the open questions above.

## Spike results (run 2026-11-02)

Harness: a scratch Rust program (monty 1.4.0 and rhai 1.27, release build, `mac_air`, M5). It is not in any repo. **The machine was saturated by other processes' compiles (load average 125-145 on 10 cores) for every run that is quoted below**, so absolute times are inflated and noisy. I ran the harness five times and kept the per-line minimum. An earlier run at load ~20 (before I made the Monty side precompile) gave a similar ratio on the host-call test, which is why I trust the ratios more than the numbers. Rerun on a quiet machine before quoting a figure.

| Question | Result |
|---|---|
| 1. Host-call round trip (script calls a host function N times, per call) | Monty ~1.4 us at N=100 and ~2.9-3.2 us at N=10,000-100,000 under heavy load; Rhai ~0.13-0.33 us. **Monty is roughly 6-20x slower per host call**; at load ~20 it was ~1.1-1.4 us against ~0.17-0.20 us. |
| Startup (compile + run `1 + 1`) | Monty 4.2 us, Rhai 1.0 us with its engine reused. Both are negligible. |
| Pure compute (sum of 1,000,000 integers) | Monty 57 ms, Rhai 153 ms. **Monty's VM is faster than Rhai's when it never leaves the sandbox.** |
| Representative trigger (loop over `persons()`, filter on affinity and membership, queue effects), precompiled | 24 persons: Monty 116 us, Rhai 13.5 us (~8.6x). 1,000 persons: Monty 4.65 ms, Rhai 1.15 ms (~4x). **Both engines queued identical effects (3 and 83) in every run.** |
| 2. Determinism, seeded | `random.seed(42)` then five draws: identical on two runs, and the first value (0.6394267984578837) is what CPython gives for seed 42, as far as I recall (not checked). Monty's `random` is a Mersenne Twister, host-seedable (`RandomStart::Seed`). |
| Determinism, unseeded | Two runs differ. A host must seed it. |
| Determinism, time limit | A loop calling the host each pass with `max_turn_duration` = 5 ms stopped after 0 to ~150,000 iterations depending on the run, and the limit overshot (7-35 ms measured against 5 ms). **A time limit is not a reproducible budget.** Rhai's operation limit stopped every run (I confirmed it stops, I did not print the counts; it counts operations, so it should be exact). |
| 3. Instruction budget | **None.** `ResourceLimits` in `monty-types` 1.4.0 has only: feed time, turn time, memory (needs `monty-alloc` as the global allocator, not tested), recursion depth, suspensions (host-enforced) and total sleep. No instruction or operation count. This would be an upstream request. |
| 4. Build | `monty` has no FFI in its dependency graph (`cargo tree` finds no pyo3). It resolves beside `bevy_society` with no duplicate serde, rhai or bevy; its MSRV is 1.96 against the family's 1.97.1. **`cargo check --target wasm32-unknown-unknown` passes** once getrandom's wasm backend is enabled (`getrandom` 0.3 feature `wasm_js` and `--cfg getrandom_backend="wasm_js"`, the same setup a Bevy web build already needs). That is a compile check only: nothing was run in a browser. |
| 5. Port | One representative trigger was ported, not the two real `village.society.ron` scripts, which are one-liners (`fx.bark(...)`, `fx.adjust(...)`) that the representative trigger's calls cover. Changes needed: `ctx.x(...)`/`fx.x(...)` became flat host functions; Rhai `let` and `#{}` maps became plain Python assignment and dicts; constants (`actor`) became assigned names. Monty can call methods on a host object, so `ctx.affinity(...)` may be possible; I did not test it. |

**Not done:** a memory-limit test (needs `monty-alloc`), `monty-pool` subprocess isolation, snapshot dump and restore, running the wasm build, and any measurement on a quiet machine.

## Batching proof-out (run 2026-11-02, quiet machine)

The first spike's Monty trigger crossed the host boundary about once per person. This run holds the trigger fixed and changes only how data crosses. A second harness (`batched.rs`, scratch, not in any repo) ran the same trigger five ways. Load average was ~2.3 and the three runs agreed to within a few percent. Every variant produced the **same effect list** (the harness asserts it). The timed region includes building the inputs and decoding the results.

Best of 7 batches, microseconds per evaluation (run 3; runs 1 and 2 were within a few percent except the 24-person Monty-snapshot cell, 7.3-7.8):

| Persons | Rhai per call | Monty per call | Monty snapshot in, effects out | Monty one coarse call |
|---|---|---|---|---|
| 24 | 5.3 | 22.9 | 7.3 | 2.8 |
| 1,000 | 197 | 835 | 197 | 25 |
| 10,000 | 1,960 | 8,309 | 1,904 | 207 |

- **Per-call Monty is ~4.2x slower than per-call Rhai at every size.** That is the gap the first spike found, reproduced on a quiet machine.
- **Passing the snapshot in and returning effects as a list (zero host calls) closes the gap**: parity with per-call Rhai at 1,000 and 10,000 persons (1.00x, 0.97x), 1.4x slower at 24 persons (7 us against 5 us, both trivial).
- **One coarse host call (`friends(actor)` does the scan natively and returns ids) is 1.8x, 7.8x and 9.5x faster than per-call Rhai.**
- Rhai given the same snapshot-in treatment was slightly slower than its own per-call form (6.5-7.0, 218-229, 2,164-2,268 us), because it must convert arrays. So the snapshot-in result is Monty matching Rhai's best-known form, not Python being intrinsically faster.

**Read this carefully.** The coarse result is mostly "the loop moved into native Rust". I did not measure a coarse call in Rhai, and it would likely be fast too. The finding is that the Python penalty comes from crossing count and goes away when crossings are batched, not that Python beats Rhai.

**Limits of this proof.** One synthetic trigger, whose host functions are trivial arithmetic (real `ctx` reads are snapshot lookups and will cost more per call). Snapshot-in builds every list in full even if a script reads three values, which wastes work at 10,000 persons; coarse calls avoid that. Scripts with more branching than a scan-and-filter may cross more.

## What the spike says

- **Python is viable as a scripting language for the Coven, at trigger rate.** Costs of a few microseconds per host call are fine for tens or hundreds of calls per trigger. They are not fine for a script that touches thousands of entities every frame, where Rust must keep the loop and the script should receive a batch.
- **The hard blocker is determinism under limits.** A seeded run reproduces. A run cut short by a time limit does not. For a simulation that promises "same seed, same result", Monty needs either an instruction-count limit upstream or a rule that no script may reach its limit (a limit hit becomes a refusal that discards all queued effects, as `bevy_society`'s scripts already do on failure).
- **Batching matters more than the interpreter.** The cheapest design for Python is the one that crosses the host boundary least: pass a snapshot in, return one effect list out.

## Decision still open

1. Does Python replace Rhai (breaking `bevy_society`), live beside it (two paths), or stay editor-side?
2. Is a time limit acceptable as the only budget, or is an upstream instruction-count limit a precondition?
3. Run the benchmark again on a quiet machine before any figure goes in a README.

## Why this is shelved

The question was asked on a preference ("Python for scripting with built-in exposures for all of our crates"), not on a problem, and the research found nothing that Python solves that the Coven lacks today.

| Likely need | Already covered by |
|---|---|
| Gameplay scripts (triggers, effects) | `bevy_society`'s Rhai: sandboxed, deterministic (an operation limit), queued writes. The real scripts in `village.society.ron` are one-liners; no Rhai limitation was found. |
| Driving the editor | The Cauldron's ~76 operations over MCP, with undo, attribution and validation. A Python script can call that MCP server from outside the process, with no embedding. |
| Blender and golem pipelines | Already Python, outside the game runtime. |

Costs of adding it: a second script path (the family's one-path rule forbids it), bindings to maintain for every crate, a Python subset, and the determinism gap under limits.

## When to revisit

Revisit when one of these is true, and not before:

- **Agents author game logic at scale.** Models write Python much better than Rhai, and Monty was built for LLM-written code. A cheap test needs no binding crate: give an agent the same trigger-writing task in Rhai and in Python and compare how often each runs correctly.
- **Outside authors or modders** who know Python are expected.
- **A concrete Rhai limitation** is hit by a person or an agent.

If it is revisited, the starting design is: scripts receive a snapshot as inputs and return one effect list, or call a few coarse host functions; bindings live in a layer above the family (the Cauldron already pins each crate as an optional feature), driven by the crates' serde types; a script that hits a limit discards all its queued effects. Re-check Monty first: it was 1.4.0 on 2026-10-26 and its limits and Python subset may have moved.

## Confidence

| Claim | Basis |
|---|---|
| Spike and batching timings | **Measured** (scratch harness, now in `2026-11-02-python-scripting-spike/`). The first spike ran at load 125-145 and is noisy; the batching run ran at load ~2.3 and is the one to trust. Neither was repeated on another machine. |
| Monty has no instruction-count limit | **Verified** from `ResourceLimits` in `monty-types` 1.4.0 source. |
| Monty has no FFI in its default dependency graph; resolves beside `bevy_society` | **Verified** with `cargo tree` and `cargo generate-lockfile`. |
| Compiles for `wasm32-unknown-unknown` | **Verified** as a compile check only; never run in a browser. |
| Python subset, sandbox model, 1.0 "ready for production" | **Reported** by Monty's own docs. Not independently tested. |
| Seeded `random` matches CPython for seed 42 | **Assumed** from memory of CPython's first value; not checked. |
| RustPython maturity, PyO3 sub-interpreter support | **Reported** by READMEs and an older issue thread; not tested. PyO3 sub-interpreter status is unconfirmed. |
| The home-still corpus has nothing on embedding Python | **Measured** by three `distill_search` queries; the corpus was not otherwise audited. |

## Reproducing

```sh
cd docs/research/2026-11-02-python-scripting-spike
export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global
cargo run --release --bin monty_spike   # first spike: startup, compute, host calls, determinism
cargo run --release --bin batched       # batching proof-out
```

Run on a quiet machine, check `uptime` first, and take the best of several runs. The `getrandom` wasm check used a separate crate depending on `monty` and `monty-types` with `getrandom` 0.3 (feature `wasm_js`) and `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`.
