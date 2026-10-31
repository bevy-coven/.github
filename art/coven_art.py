"""Generate pixel-art candidates for the bevy-coven org avatar and banner (runs on big).

Uses the local Retro Diffusion checkpoints in /mnt/codex_fs/weights through the pipeline from
Sanctuary's icon work (~/sanctuary-icons/rd_generate.py, read-only). Run it under the GPU lease:

    gpu-tenant run 3d --who bevy-coven-art@big --wait --mem 16G --note "bevy-coven org art" -- \
        ~/sanctuary-icons/.venv/bin/python ~/coven-art/coven_art.py --jobs JOBS.json --out DIR

JOBS.json: {"model", "width_px", "height_px", "steps", "guidance", "expected_factor",
"palette_colors", "out_scale", "prompt", "negative", "seeds_per_item",
"items": [{"id", "subject", "seed"}]}. Candidate k of an item uses seed + k.

Per candidate: generate at width_px x height_px, recover the pixel grid with pixelfixer (kept when its
width is within 25% of width_px / expected_factor, else a block reduce by expected_factor, flagged
`block`), keep the full frame, quantize to palette_colors, scale up by out_scale with NEAREST. Writes
raw/, candidates/<id>-<k>.png, results.json and sheet.png under DIR.
"""

import argparse
import collections
import json
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path.home() / "sanctuary-icons"))
import rd_generate  # noqa: E402
from PIL import Image, ImageDraw, ImageFont  # noqa: E402

rd_generate.MODEL_DIR = Path("/mnt/codex_fs/weights")

SQUARE_VIEW_PX = 256
THUMB_PX = 40
BANNER_VIEW_W = 576
SHEET_BG = (20, 20, 22)


def generate(pipe, job, subject, seed):
    import torch

    return pipe(
        prompt=job["prompt"].format(subject=subject),
        negative_prompt=job["negative"],
        width=job["width_px"],
        height=job["height_px"],
        num_inference_steps=job["steps"],
        guidance_scale=job["guidance"],
        generator=torch.Generator("cuda").manual_seed(seed),
    ).images[0]


def block_reduce(img, f):
    """Each output pixel is the most common RGB of its f x f block, for any aspect ratio."""
    rgb = img.convert("RGB")
    w, h = rgb.width // f, rgb.height // f
    out = Image.new("RGBA", (w, h))
    for y in range(h):
        for x in range(w):
            block = rgb.crop((x * f, y * f, x * f + f, y * f + f))
            colour = collections.Counter(block.getdata()).most_common(1)[0][0]
            out.putpixel((x, y), (*colour, 255))
    return out


def native_pixels(raw, raw_path, job):
    """Returns (RGBA native image, method, flags)."""
    expected_w = job["width_px"] / job["expected_factor"]
    with tempfile.TemporaryDirectory() as tmp:
        out_path = Path(tmp) / "native.png"
        info, why = rd_generate.run_pixelfixer(raw_path, out_path)
        if info is not None and out_path.exists():
            cols = int(info["cols"])
            if abs(cols - expected_w) <= 0.25 * expected_w:
                with Image.open(out_path) as native:
                    return native.convert("RGBA"), "pixelfixer", []
            why = f"grid {cols}x{info['rows']}"
    print(f"  pixelfixer rejected ({why}); block reduce", flush=True)
    return block_reduce(raw, job["expected_factor"]), "block", ["block"]


def opaque(img):
    rgba = img.convert("RGBA")
    rgba.putalpha(255)
    return rgba


def render_sheet(cells, path, square):
    """cells: [(RGBA image, label)]."""
    font = ImageFont.load_default()
    if square:
        cell_w, cell_h, cols = SQUARE_VIEW_PX + THUMB_PX + 24, SQUARE_VIEW_PX + 20, 4
    else:
        cell_w, cell_h, cols = BANNER_VIEW_W + 16, BANNER_VIEW_W // 3 + 24, 2
    rows = (len(cells) + cols - 1) // cols
    sheet = Image.new("RGB", (cols * cell_w, max(1, rows) * cell_h), (0, 0, 0))
    draw = ImageDraw.Draw(sheet)
    for i, (img, label) in enumerate(cells):
        x, y = (i % cols) * cell_w, (i // cols) * cell_h
        draw.rectangle((x, y, x + cell_w - 1, y + cell_h - 1), fill=SHEET_BG)
        if square:
            view = img.resize((SQUARE_VIEW_PX, SQUARE_VIEW_PX), Image.Resampling.NEAREST)
            thumb = img.resize((THUMB_PX, THUMB_PX), Image.Resampling.LANCZOS)
            sheet.paste(view.convert("RGB"), (x + 4, y + 4))
            sheet.paste(thumb.convert("RGB"), (x + SQUARE_VIEW_PX + 12, y + 4))
            draw.text((x + 4, y + SQUARE_VIEW_PX + 6), label, fill=(230, 230, 230), font=font)
        else:
            h = round(img.height * BANNER_VIEW_W / img.width)
            view = img.resize((BANNER_VIEW_W, h), Image.Resampling.NEAREST)
            sheet.paste(view.convert("RGB"), (x + 8, y + 4))
            draw.text((x + 8, y + h + 8), label, fill=(230, 230, 230), font=font)
    sheet.save(path)


def run(job, out):
    (out / "raw").mkdir(parents=True, exist_ok=True)
    (out / "candidates").mkdir(parents=True, exist_ok=True)
    pipe = rd_generate.load_pipeline(job["model"])
    results, cells = [], []
    for item in job["items"]:
        for k in range(job["seeds_per_item"]):
            seed = item["seed"] + k
            name = f"{item['id']}-{k}"
            raw = generate(pipe, job, item["subject"], seed)
            raw_path = out / "raw" / f"{name}.png"
            raw.save(raw_path)
            native, method, flags = native_pixels(raw, raw_path, job)
            frame = rd_generate.quantize(opaque(native), job["palette_colors"])
            scaled = frame.resize(
                (frame.width * job["out_scale"], frame.height * job["out_scale"]),
                Image.Resampling.NEAREST,
            )
            scaled.save(out / "candidates" / f"{name}.png")
            results.append(
                {"id": item["id"], "k": k, "seed": seed, "native": list(native.size),
                 "method": method, "flags": flags}
            )
            cells.append((scaled, f"{name} s{seed}" + (" [block]" if flags else "")))
            print(f"{name} seed={seed} native={native.size[0]}x{native.size[1]} {method}", flush=True)
    (out / "results.json").write_text(json.dumps(results, indent=1))
    render_sheet(cells, out / "sheet.png", job["width_px"] == job["height_px"])
    print(f"wrote {out / 'sheet.png'}", flush=True)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--jobs", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    job = json.loads(Path(args.jobs).read_text())
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    run(job, out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
