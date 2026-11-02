//! Spike: Monty (Python) vs Rhai as a Coven scripting host. Scratch code, not for the repo.
//! Measures call overhead, startup, determinism, and a ported bevy_society-shaped trigger.

use monty::{MontyRun, RunProgress};
use monty_types::{CompileOptions, MontyObject, PrintWriter, ResourceLimits, ResourceTracker};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

const VILLAGE_SIZE: usize = 24;
const CROWD_SIZE: usize = 1000;
const HOST_CALL_COUNTS: [usize; 4] = [1, 100, 10_000, 100_000];
const COMPUTE_ITERATIONS: i64 = 1_000_000;
const STARTUP_REPEATS: u32 = 2_000;
const TRIGGER_REPEATS: u32 = 200;
const TIME_LIMIT_RUNS: usize = 20;
const TIME_LIMIT_MS: u64 = 5;

/// Deterministic stand-in for `ctx.affinity(a, b)`: pure arithmetic on the ids.
fn affinity(a: i64, b: i64) -> f64 {
    (((a * 31 + b * 17) % 200) as f64) / 100.0 - 1.0
}
fn member(a: i64) -> bool {
    a % 3 == 0
}

#[derive(Debug, Clone, PartialEq)]
enum Fx {
    Adjust(i64, i64, f64),
    Bark(i64),
}

// ---------------------------------------------------------------- Monty host

fn compile_monty(src: &str, inputs: &[&str]) -> MontyRun {
    let names: Vec<String> = inputs.iter().map(|s| s.to_string()).collect();
    MontyRun::new(src.to_owned(), "spike.py", names, CompileOptions::default()).expect("compile")
}

/// Compile + drive, for one-shot uses.
fn run_monty(
    src: &str,
    inputs: &[&str],
    persons: i64,
    limits: ResourceLimits,
    fx: &mut Vec<Fx>,
    tick: &mut dyn FnMut(i64),
) -> Result<MontyObject, String> {
    drive_monty(compile_monty(src, inputs), inputs, persons, limits, fx, tick)
}

/// Drives a precompiled Monty script to completion, answering host calls from the mock society.
fn drive_monty(
    runner: MontyRun,
    inputs: &[&str],
    persons: i64,
    limits: ResourceLimits,
    fx: &mut Vec<Fx>,
    tick: &mut dyn FnMut(i64),
) -> Result<MontyObject, String> {
    let args: Vec<MontyObject> = inputs.iter().map(|n| MontyObject::function(n.to_string(), None)).collect();
    let mut progress = runner
        .start(args,ResourceTracker::new(limits), PrintWriter::Disabled)
        .map_err(|e| e.to_string())?;
    loop {
        match progress {
            RunProgress::Complete(v) => return Ok(v),
            RunProgress::FunctionCall(call) => {
                let int = |i: usize| call.args.arg(i).and_then(|o| o.as_int()).unwrap_or(-1);
                let reply = match call.function_name.as_str() {
                    "affinity" => MontyObject::float(affinity(int(0), int(1))),
                    "member" => MontyObject::bool(member(int(0))),
                    "persons" => MontyObject::list((0..persons).map(MontyObject::int)),
                    "adjust" => {
                        let d = call.args.arg(2).and_then(|o| o.as_float()).unwrap_or(0.0);
                        fx.push(Fx::Adjust(int(0), int(1), d));
                        MontyObject::none()
                    }
                    "bark" => {
                        fx.push(Fx::Bark(int(0)));
                        MontyObject::none()
                    }
                    "tick" => {
                        tick(int(0));
                        MontyObject::none()
                    }
                    "f" => MontyObject::int(int(0)),
                    other => return Err(format!("unknown host fn {other}")),
                };
                progress = call.resume(reply, PrintWriter::Disabled).map_err(|e| e.to_string())?;
            }
            other => return Err(format!("unexpected progress: {}", std::any::type_name_of_val(&other))),
        }
    }
}

// ---------------------------------------------------------------- Rhai host

fn rhai_engine(persons: i64, fx: Rc<RefCell<Vec<Fx>>>) -> rhai::Engine {
    let mut e = rhai::Engine::new();
    e.set_max_operations(0); // unlimited here: measuring cost, not limits
    e.register_fn("affinity", |a: i64, b: i64| affinity(a, b));
    e.register_fn("member", |a: i64| member(a));
    e.register_fn("persons", move || (0..persons).map(rhai::Dynamic::from).collect::<rhai::Array>());
    let f1 = fx.clone();
    e.register_fn("adjust", move |a: i64, b: i64, d: f64| f1.borrow_mut().push(Fx::Adjust(a, b, d)));
    let f2 = fx.clone();
    e.register_fn("bark", move |a: i64| f2.borrow_mut().push(Fx::Bark(a)));
    e.register_fn("f", |i: i64| i);
    e
}

fn per(d: Duration, n: usize) -> f64 {
    d.as_secs_f64() * 1e9 / n as f64
}

fn main() {
    println!("== T1 startup: compile + run `1 + 1`, mean over {STARTUP_REPEATS} ==");
    let t = Instant::now();
    for _ in 0..STARTUP_REPEATS {
        let mut fx = vec![];
        run_monty("1 + 1", &[], 0, ResourceLimits::default(), &mut fx, &mut |_| {}).unwrap();
    }
    let monty_start = t.elapsed();
    let engine = rhai::Engine::new();
    let t = Instant::now();
    for _ in 0..STARTUP_REPEATS {
        let _: i64 = engine.eval("1 + 1").unwrap();
    }
    let rhai_start = t.elapsed();
    println!(
        "monty {:.1} us/run   rhai (engine reused) {:.1} us/run",
        per(monty_start, STARTUP_REPEATS as usize) / 1e3,
        per(rhai_start, STARTUP_REPEATS as usize) / 1e3
    );

    println!("\n== T2 pure compute: sum of range({COMPUTE_ITERATIONS}) ==");
    let mut fx = vec![];
    let t = Instant::now();
    let v = run_monty(
        &format!("s = 0\nfor i in range({COMPUTE_ITERATIONS}):\n    s += i\ns"),
        &[],
        0,
        ResourceLimits::default(),
        &mut fx,
        &mut |_| {},
    )
    .unwrap();
    println!("monty {:.1} ms (result {v:?})", t.elapsed().as_secs_f64() * 1e3);
    let t = Instant::now();
    let r: i64 = engine
        .eval(&format!("let s = 0; for i in 0..{COMPUTE_ITERATIONS} {{ s += i; }} s"))
        .unwrap();
    println!("rhai  {:.1} ms (result {r})", t.elapsed().as_secs_f64() * 1e3);

    println!("\n== T3 host-call round trip: script calls f(i) N times ==");
    for n in HOST_CALL_COUNTS {
        let mut fx = vec![];
        let compiled = compile_monty(&format!("for i in range({n}):\n    f(i)"), &["f"]);
        let t = Instant::now();
        drive_monty(
            compiled,
            &["f"],
            0,
            ResourceLimits { max_suspensions: usize::MAX, ..ResourceLimits::default() },
            &mut fx,
            &mut |_| {},
        )
        .unwrap();
        let m = t.elapsed();
        let fxr = Rc::new(RefCell::new(vec![]));
        let e = rhai_engine(0, fxr);
        let t = Instant::now();
        e.eval::<()>(&format!("for i in 0..{n} {{ f(i); }}")).unwrap();
        let r = t.elapsed();
        println!("N={n:>7}: monty {:>9.0} ns/call   rhai {:>7.0} ns/call", per(m, n), per(r, n));
    }

    println!("\n== T4 ported trigger: 'friends of actor within crowd' (loop persons, filter on affinity, queue effects) ==");
    let monty_src = "bark_done = False\nfor p in persons():\n    if p != actor and affinity(actor, p) > 0.5 and member(p):\n        adjust(actor, p, 0.1)\n        if not bark_done:\n            bark(actor)\n            bark_done = True\nNone";
    let rhai_src = "let done = false; for p in persons() { if p != actor && affinity(actor, p) > 0.5 && member(p) { adjust(actor, p, 0.1); if !done { bark(actor); done = true; } } }";
    for size in [VILLAGE_SIZE, CROWD_SIZE] {
        let mut mfx_last = vec![];
        let host_fns = ["persons", "affinity", "member", "adjust", "bark"];
        let compiled = compile_monty(&format!("actor = 5\n{monty_src}"), &host_fns);
        let t = Instant::now();
        for _ in 0..TRIGGER_REPEATS {
            mfx_last = vec![];
            drive_monty(
                compiled.clone(),
                &host_fns,
                size as i64,
                ResourceLimits { max_suspensions: usize::MAX, ..ResourceLimits::default() },
                &mut mfx_last,
                &mut |_| {},
            )
            .unwrap();
        }
        let m = t.elapsed();
        let fxr = Rc::new(RefCell::new(vec![]));
        let e = rhai_engine(size as i64, fxr.clone());
        let ast = e.compile(rhai_src).unwrap();
        let mut last_len = 0;
        let t = Instant::now();
        for _ in 0..TRIGGER_REPEATS {
            fxr.borrow_mut().clear();
            let mut scope = rhai::Scope::new();
            scope.push_constant("actor", 5_i64);
            e.run_ast_with_scope(&mut scope, &ast).unwrap();
            last_len = fxr.borrow().len();
        }
        let r = t.elapsed();
        let same = *fxr.borrow() == mfx_last;
        println!(
            "persons={size:>5}: monty {:>8.1} us/eval   rhai {:>8.1} us/eval   effects monty={} rhai={last_len} identical={same}",
            per(m, TRIGGER_REPEATS as usize) / 1e3,
            per(r, TRIGGER_REPEATS as usize) / 1e3,
            mfx_last.len()
        );
    }

    println!("\n== T5a determinism: seeded random, same script twice ==");
    let seeded = "import random\nrandom.seed(42)\n[random.random() for _ in range(5)]";
    let mut fx = vec![];
    let a = run_monty(seeded, &[], 0, ResourceLimits::default(), &mut fx, &mut |_| {}).unwrap();
    let b = run_monty(seeded, &[], 0, ResourceLimits::default(), &mut fx, &mut |_| {}).unwrap();
    println!("seeded identical: {}   value {a:?}", a == b);
    let unseeded = "import random\n[random.random() for _ in range(3)]";
    let a = run_monty(unseeded, &[], 0, ResourceLimits::default(), &mut fx, &mut |_| {});
    let b = run_monty(unseeded, &[], 0, ResourceLimits::default(), &mut fx, &mut |_| {});
    println!("unseeded identical: {}", a == b);

    println!("\n== T5b time-limit determinism: loop calling tick(i), max_turn_duration={TIME_LIMIT_MS} ms, {TIME_LIMIT_RUNS} runs ==");
    let mut reached = vec![];
    for _ in 0..TIME_LIMIT_RUNS {
        let mut last = 0_i64;
        let mut fx = vec![];
        let res = run_monty(
            "i = 0\nwhile True:\n    i += 1\n    tick(i)",
            &["tick"],
            0,
            ResourceLimits {
                max_turn_duration: Some(Duration::from_millis(TIME_LIMIT_MS)),
                max_suspensions: usize::MAX,
                ..ResourceLimits::default()
            },
            &mut fx,
            &mut |i| last = i,
        );
        let err = res.expect_err("loop should have been stopped");
        if reached.is_empty() { println!("stop reason: {}", err.lines().last().unwrap_or("")); }
        reached.push(last);
    }
    // NOTE: max_turn_duration resets at each resume, so a loop that calls the host every iteration may never trip it.
    println!("iterations reached before stop: min {} max {} (all-equal={})",
        reached.iter().min().unwrap(), reached.iter().max().unwrap(), reached.iter().all(|x| *x == reached[0]));

    println!("\n== T5c time limit with no host calls (pure loop), counting via side effect is impossible, so report error kind ==");
    let mut fx = vec![];
    let t = Instant::now();
    let r = run_monty(
        "n = 0\nwhile True:\n    n += 1",
        &[],
        0,
        ResourceLimits { max_feed_duration: Some(Duration::from_millis(TIME_LIMIT_MS)), ..ResourceLimits::default() },
        &mut fx,
        &mut |_| {},
    );
    println!("stopped after {:.1} ms with: {}", t.elapsed().as_secs_f64() * 1e3, r.unwrap_err().lines().last().unwrap_or(""));

    println!("\n== T6 rhai op-count limit determinism: loop to limit 10_000 ops, 5 runs ==");
    let mut counts = vec![];
    for _ in 0..5 {
        let mut e = rhai::Engine::new();
        e.set_max_operations(10_000);
        let r = e.eval::<i64>("let n = 0; loop { n += 1; }");
        counts.push(r.is_err());
    }
    println!("all stopped by op limit: {}", counts.iter().all(|x| *x));
}
