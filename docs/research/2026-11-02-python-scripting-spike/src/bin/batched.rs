//! Proof-out: does batching close the Monty-vs-Rhai gap on the same trigger?
//! Variants (all produce the same effect list, asserted):
//!   rhai_percall   host functions called per person (the existing Rhai style)
//!   rhai_snapshot  snapshot passed in as arrays, one run
//!   monty_percall  host functions called per person (the first spike's style)
//!   monty_snapshot snapshot passed in as inputs, effects returned as a list: zero host calls
//!   monty_coarse   one coarse host call `friends(actor)`, effects returned as a list
//! Timed region includes building the inputs and decoding the result, because a real host must do both.

use monty::{MontyRun, RunProgress};
use monty_types::{CompileOptions, MontyObject, PrintWriter, ResourceLimits, ResourceTracker};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

const ACTOR: i64 = 5;
const AFFINITY_MIN: f64 = 0.5;
const ADJUST_DELTA: f64 = 0.1;
const SIZES: [usize; 3] = [24, 1000, 10_000];
const BATCHES: usize = 7;

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

/// Best (lowest) per-evaluation time in microseconds over `BATCHES` batches of `reps`.
fn best_us(reps: usize, mut f: impl FnMut()) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..BATCHES {
        let t = Instant::now();
        for _ in 0..reps {
            f();
        }
        best = best.min(t.elapsed().as_secs_f64() * 1e6 / reps as f64);
    }
    best
}

// ---------------------------------------------------------------- Monty

const MONTY_PERCALL: &str = "actor = 5\ndone = False\nfor p in persons():\n    if p != actor and affinity(actor, p) > 0.5 and member(p):\n        adjust(actor, p, 0.1)\n        if not done:\n            bark(actor)\n            done = True\nNone";
const MONTY_SNAPSHOT: &str = "out = []\ndone = False\nfor i in range(len(ids)):\n    p = ids[i]\n    if p != actor and aff[i] > 0.5 and mem[i]:\n        out.append(('adjust', actor, p, 0.1))\n        if not done:\n            out.append(('bark', actor, -1, 0.0))\n            done = True\nout";
const MONTY_COARSE: &str = "out = []\ndone = False\nfor p in friends(actor):\n    out.append(('adjust', actor, p, 0.1))\n    if not done:\n        out.append(('bark', actor, -1, 0.0))\n        done = True\nout";

fn compile(src: &str, inputs: &[&str]) -> MontyRun {
    MontyRun::new(src.to_owned(), "t.py", inputs.iter().map(|s| s.to_string()).collect(), CompileOptions::default())
        .expect("compile")
}

fn decode(result: &MontyObject) -> Vec<Fx> {
    let mut out = vec![];
    for row in result.as_ref().items().expect("list") {
        let f = row.items().expect("tuple");
        let kind = f[0].as_str().expect("kind");
        let a = f[1].as_int().expect("a");
        match kind {
            "adjust" => out.push(Fx::Adjust(a, f[2].as_int().expect("b"), f[3].as_float().expect("d"))),
            "bark" => out.push(Fx::Bark(a)),
            other => panic!("unknown effect {other}"),
        }
    }
    out
}

fn monty_percall(runner: &MontyRun, n: i64) -> Vec<Fx> {
    let names = ["persons", "affinity", "member", "adjust", "bark"];
    let args: Vec<MontyObject> = names.iter().map(|x| MontyObject::function(x.to_string(), None)).collect();
    let mut fx = vec![];
    let mut progress = runner
        .clone()
        .start(args, ResourceTracker::new(ResourceLimits { max_suspensions: usize::MAX, ..Default::default() }), PrintWriter::Disabled)
        .expect("start");
    loop {
        match progress {
            RunProgress::Complete(_) => return fx,
            RunProgress::FunctionCall(call) => {
                let int = |i: usize| call.args.arg(i).and_then(|o| o.as_int()).unwrap_or(-1);
                let reply = match call.function_name.as_str() {
                    "affinity" => MontyObject::float(affinity(int(0), int(1))),
                    "member" => MontyObject::bool(member(int(0))),
                    "persons" => MontyObject::list((0..n).map(MontyObject::int)),
                    "adjust" => {
                        let d = call.args.arg(2).and_then(|o| o.as_float()).unwrap_or(0.0);
                        fx.push(Fx::Adjust(int(0), int(1), d));
                        MontyObject::none()
                    }
                    "bark" => {
                        fx.push(Fx::Bark(int(0)));
                        MontyObject::none()
                    }
                    other => panic!("host fn {other}"),
                };
                progress = call.resume(reply, PrintWriter::Disabled).expect("resume");
            }
            _ => panic!("unexpected progress"),
        }
    }
}

fn monty_snapshot(runner: &MontyRun, n: i64) -> Vec<Fx> {
    let inputs = vec![
        MontyObject::int(ACTOR),
        MontyObject::list((0..n).map(MontyObject::int)),
        MontyObject::list((0..n).map(|p| MontyObject::float(affinity(ACTOR, p)))),
        MontyObject::list((0..n).map(|p| MontyObject::bool(member(p)))),
    ];
    let mut r = runner.clone();
    let result = r.run(inputs, ResourceTracker::default(), PrintWriter::Disabled).expect("run");
    decode(&result)
}

fn monty_coarse(runner: &MontyRun, n: i64) -> Vec<Fx> {
    let args = vec![MontyObject::int(ACTOR), MontyObject::function("friends".to_owned(), None)];
    let mut progress = runner.clone().start(args, ResourceTracker::default(), PrintWriter::Disabled).expect("start");
    loop {
        match progress {
            RunProgress::Complete(v) => return decode(&v),
            RunProgress::FunctionCall(call) => {
                assert_eq!(call.function_name, "friends");
                let actor = call.args.arg(0).and_then(|o| o.as_int()).unwrap_or(-1);
                let hits = (0..n).filter(|p| *p != actor && affinity(actor, *p) > AFFINITY_MIN && member(*p));
                progress = call.resume(MontyObject::list(hits.map(MontyObject::int)), PrintWriter::Disabled).expect("resume");
            }
            _ => panic!("unexpected progress"),
        }
    }
}

// ---------------------------------------------------------------- Rhai

const RHAI_PERCALL: &str = "let done = false; for p in persons() { if p != actor && affinity(actor, p) > 0.5 && member(p) { adjust(actor, p, 0.1); if !done { bark(actor); done = true; } } }";
const RHAI_SNAPSHOT: &str = "let out = []; let done = false; for i in 0..ids.len() { let p = ids[i]; if p != actor && aff[i] > 0.5 && mem[i] { out.push([0, actor, p, 0.1]); if !done { out.push([1, actor, -1, 0.0]); done = true; } } } out";

fn rhai_percall_engine(n: i64, fx: Rc<RefCell<Vec<Fx>>>) -> rhai::Engine {
    let mut e = rhai::Engine::new();
    e.register_fn("affinity", |a: i64, b: i64| affinity(a, b));
    e.register_fn("member", |a: i64| member(a));
    e.register_fn("persons", move || (0..n).map(rhai::Dynamic::from).collect::<rhai::Array>());
    let f1 = fx.clone();
    e.register_fn("adjust", move |a: i64, b: i64, d: f64| f1.borrow_mut().push(Fx::Adjust(a, b, d)));
    let f2 = fx;
    e.register_fn("bark", move |a: i64| f2.borrow_mut().push(Fx::Bark(a)));
    e
}

fn rhai_snapshot(e: &rhai::Engine, ast: &rhai::AST, n: i64) -> Vec<Fx> {
    let mut scope = rhai::Scope::new();
    scope.push_constant("actor", ACTOR);
    scope.push_constant("ids", (0..n).map(rhai::Dynamic::from).collect::<rhai::Array>());
    scope.push_constant("aff", (0..n).map(|p| rhai::Dynamic::from(affinity(ACTOR, p))).collect::<rhai::Array>());
    scope.push_constant("mem", (0..n).map(|p| rhai::Dynamic::from(member(p))).collect::<rhai::Array>());
    let out: rhai::Array = e.eval_ast_with_scope(&mut scope, ast).expect("rhai");
    out.into_iter()
        .map(|row| {
            let r = row.cast::<rhai::Array>();
            if r[0].as_int().unwrap() == 0 {
                Fx::Adjust(r[1].as_int().unwrap(), r[2].as_int().unwrap(), r[3].as_float().unwrap())
            } else {
                Fx::Bark(r[1].as_int().unwrap())
            }
        })
        .collect()
}

fn main() {
    println!("best of {BATCHES} batches; timed region includes building inputs and decoding results\n");
    println!("{:>7} | {:>13} {:>13} | {:>13} {:>14} {:>13} | monty_snapshot / rhai_percall", "persons", "rhai_percall", "rhai_snapshot", "monty_percall", "monty_snapshot", "monty_coarse");
    for size in SIZES {
        let n = size as i64;
        let reps = (200_000 / size).clamp(5, 400);

        let fxr = Rc::new(RefCell::new(vec![]));
        let e1 = rhai_percall_engine(n, fxr.clone());
        let ast1 = e1.compile(RHAI_PERCALL).unwrap();
        let reference = {
            fxr.borrow_mut().clear();
            let mut s = rhai::Scope::new();
            s.push_constant("actor", ACTOR);
            e1.run_ast_with_scope(&mut s, &ast1).unwrap();
            fxr.borrow().clone()
        };
        let rhai_pc = best_us(reps, || {
            fxr.borrow_mut().clear();
            let mut s = rhai::Scope::new();
            s.push_constant("actor", ACTOR);
            e1.run_ast_with_scope(&mut s, &ast1).unwrap();
        });

        let e2 = rhai::Engine::new();
        let ast2 = e2.compile(RHAI_SNAPSHOT).unwrap();
        assert_eq!(rhai_snapshot(&e2, &ast2, n), reference, "rhai_snapshot effects differ");
        let rhai_sn = best_us(reps, || {
            std::hint::black_box(rhai_snapshot(&e2, &ast2, n));
        });

        let m1 = compile(MONTY_PERCALL, &["persons", "affinity", "member", "adjust", "bark"]);
        assert_eq!(monty_percall(&m1, n), reference, "monty_percall effects differ");
        let monty_pc = best_us(reps, || {
            std::hint::black_box(monty_percall(&m1, n));
        });

        let m2 = compile(MONTY_SNAPSHOT, &["actor", "ids", "aff", "mem"]);
        assert_eq!(monty_snapshot(&m2, n), reference, "monty_snapshot effects differ");
        let monty_sn = best_us(reps, || {
            std::hint::black_box(monty_snapshot(&m2, n));
        });

        let m3 = compile(MONTY_COARSE, &["actor", "friends"]);
        assert_eq!(monty_coarse(&m3, n), reference, "monty_coarse effects differ");
        let monty_co = best_us(reps, || {
            std::hint::black_box(monty_coarse(&m3, n));
        });

        println!(
            "{size:>7} | {rhai_pc:>10.1} us {rhai_sn:>10.1} us | {monty_pc:>10.1} us {monty_sn:>11.1} us {monty_co:>10.1} us | {:.2}x   (effects: {})",
            monty_sn / rhai_pc,
            reference.len()
        );
    }
}
