# Org art

The bevy-coven avatar and banner: pixel art made on big with the local Retro Diffusion checkpoints (`/mnt/codex_fs/weights/RetroDiffusion*Model.safetensors`, SD 1.5), through the pipeline from Sanctuary's icon work (decision 0057). Motifs: dark magic and 80s airbrushed custom-van murals.

- `coven_art.py`: the generator. Its docstring has the job schema.
- `avatar.json`, `banner.json`, `banner-round2.json`: the jobs that were run.
- `avatar.png` (512×512) and `banner.png` (1152×384): the winners. `../profile/banner.png` is a copy of the banner.
- `avatar-sheet.png`, `banner-sheet-round1.png`, `banner-sheet-round2.png`: every candidate, for review.

## Rerun

Copy `coven_art.py` and the job file to `big:~/coven-art/`, then:

```sh
ssh big "bash -lc 'gpu-tenant run 3d --who bevy-coven-art@big --wait --mem 16G --note \"bevy-coven org art\" -- ~/sanctuary-icons/.venv/bin/python ~/coven-art/coven_art.py --jobs ~/coven-art/avatar.json --out ~/coven-art/runs/avatar-1'"
ssh big "bash -lc 'gpu-tenant run 3d --who bevy-coven-art@big --wait --mem 16G --note \"bevy-coven org art\" -- ~/sanctuary-icons/.venv/bin/python ~/coven-art/coven_art.py --jobs ~/coven-art/banner-round2.json --out ~/coven-art/runs/banner-round2-2'"
```

The avatar winner is `runs/avatar-1/candidates/raven_grimoire-1.png` (seed 1002); the banner winner is `runs/banner-round2-2/candidates/wizard_cliff-2.png` (seed 7203). Generation is seeded, so the same checkpoint, diffusers version and GPU give the same image.

## How the winners were picked

Each candidate scored 0-2 on five criteria, 10 in all:

- **a** the subject reads at a 40 px thumbnail (banner: at full size);
- **b** one clear subject with a strong silhouette against the sky;
- **c** no text-like glyphs, pentagrams or gore;
- **d** the motif: moon, purple-to-orange gradient, lightning or chrome;
- **e** a clean pixel grid: no `block` fallback, no smeared pixels.

The highest total wins; a tie goes to a raven, because the raven is the familiar.

### Avatar (round 1, 10 candidates; `RetroDiffusion24x-64xModel`, 512 px, 64×64 native)

|Candidate|Seed|a|b|c|d|e|Total|
|---|---|---|---|---|---|---|---|
|raven_grimoire-0|1001|2|2|2|1|2|9|
|**raven_grimoire-1**|1002|2|2|2|2|2|**10**|
|witch_moon-0|2001|1|0|2|1|2|6|
|witch_moon-1|2002|2|2|2|2|2|10|
|cat_grimoire-0|3001|1|1|2|1|2|7|
|cat_grimoire-1|3002|2|2|2|1|2|9|
|grimoire_runes-0|4001|1|1|1|2|2|7|
|grimoire_runes-1|4002|1|1|1|2|2|7|
|raven_hat-0|5001|2|2|2|1|2|9|
|raven_hat-1|5002|1|1|2|2|2|8|

`raven_grimoire-1` and `witch_moon-1` tie at 10; the raven wins the tie. It is a black raven with purple wings against a yellow-and-orange moon, ringed in purple.

### Banner, round 1 (6 candidates; `RetroDiffusion128xModel`, 768×256, `expected_factor` 4)

Pixelfixer recovered a 96×32 grid on all six, a factor of 8 rather than the uncalibrated guess of 4, so every candidate fell back to a block reduce and scored 0 on **e**.

|Candidate|Seed|a|b|c|d|e|Total|
|---|---|---|---|---|---|---|---|
|coven_altar-0|6001|1|1|2|1|0|5|
|coven_altar-1|6002|1|1|1|0|0|3|
|wizard_cliff-0|7001|1|1|2|1|0|5|
|wizard_cliff-1|7002|1|1|2|1|0|5|
|castle_brooms-0|8001|2|2|2|1|0|7|
|castle_brooms-1|8002|1|1|2|1|0|5|

### Banner, round 2 (6 candidates; the two best subjects, seeds `seed + 200`, `expected_factor` 8, `out_scale` 12)

Round 1's best reached the threshold of 7 only because its grid was wrong, so round 2 reran the two best subjects at the measured factor of 8.

|Candidate|Seed|a|b|c|d|e|Total|
|---|---|---|---|---|---|---|---|
|castle_brooms-0|8201|1|1|2|1|2|7|
|castle_brooms-1|8202|2|1|2|2|2|9|
|castle_brooms-2|8203|2|2|2|2|2|10|
|wizard_cliff-0|7201|1|0|2|1|2|6|
|wizard_cliff-1|7202|1|1|2|1|2|7|
|**wizard_cliff-2**|7203|2|2|2|2|2|**10**|

`wizard_cliff-2` and `castle_brooms-2` tie at 10; the banner goes to `wizard_cliff-2`, a dark raven-winged figure on a rock under a full moon, with airbrushed purple, orange and yellow cloud bands and a lightning streak, because its figure is a raven.
