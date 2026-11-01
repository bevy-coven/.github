#!/usr/bin/env python3
"""Minimal Retro Diffusion v2 client: python3 rd_gen.py JOBS.json OUTDIR [--cost-only]
Key comes from ~/.config/retrodiffusion/api_key (never from argv, never into a repo).
A job: {"name": ..., "prompt": ..., "prompt_style": ..., "width": N, "height": N, "num_images": N, "seed"?: N,
        "reference": "path.png"?}  Reference images are sent base64 in reference_images."""
import base64, json, os, subprocess, sys, time, uuid

BASE = "https://api.retrodiffusion.ai/v2/inferences"
KEY = open(os.path.expanduser("~/.config/retrodiffusion/api_key")).read().strip()


def call(method, url, body=None, idem=None):
    cmd = ["curl", "-sS", "-X", method, url, "-H", f"X-RD-Token: {KEY}", "-H", "Content-Type: application/json"]
    if idem:
        cmd += ["-H", f"Idempotency-Key: {idem}"]
    if body is not None:
        cmd += ["--data-binary", "@-"]
    out = subprocess.run(cmd, input=json.dumps(body) if body is not None else None, capture_output=True, text=True).stdout
    try:
        return json.loads(out)
    except Exception:
        raise SystemExit(f"bad response: {out[:300]}")


def payload(job):
    p = {k: job[k] for k in ("prompt", "prompt_style", "width", "height", "num_images") if k in job}
    for k in ("seed", "remove_bg"):
        if k in job:
            p[k] = job[k]
    if "reference" in job:
        p["reference_images"] = [base64.b64encode(open(job["reference"], "rb").read()).decode()]
    return p


def main():
    jobs = json.load(open(sys.argv[1]))
    out = sys.argv[2]
    cost_only = "--cost-only" in sys.argv
    os.makedirs(out, exist_ok=True)
    total = 0.0
    pending = []
    for job in jobs:
        p = payload(job)
        c = call("POST", BASE, {**p, "check_cost": True})
        cost = c.get("balance_cost")
        print(f"{job['name']}: ${cost}", flush=True)
        total += cost or 0
    print(f"total ${total:.2f}")
    if cost_only:
        return
    for job in jobs:
        idem = str(uuid.uuid4())
        open(os.path.join(out, job["name"] + ".idem"), "w").write(idem)
        acc = call("POST", BASE, payload(job), idem=idem)
        if "task_id" not in acc:
            print(job["name"], "REFUSED", acc)
            continue
        pending.append((job, acc["task_id"]))
        while len(pending) >= 2:  # the API allows two concurrent slots
            pending = drain(pending, out)
            time.sleep(2)
    while pending:
        pending = drain(pending, out)
        time.sleep(2)


def drain(pending, out):
    left = []
    for job, tid in pending:
        t = call("GET", f"{BASE}/tasks/{tid}")
        st = t.get("status")
        if st in ("pending", "running", "accepted"):
            left.append((job, tid))
        elif st == "failed":
            print(job["name"], "FAILED", t.get("error"))
        else:
            r = t["result"]
            for i, b in enumerate(r["base64_images"]):
                path = os.path.join(out, f"{job['name']}-{i}.png")
                open(path, "wb").write(base64.b64decode(b))
            print(f"{job['name']}: {len(r['base64_images'])} image(s), cost ${r.get('balance_cost')}, balance ${r.get('remaining_balance')}", flush=True)
    return left


main()
