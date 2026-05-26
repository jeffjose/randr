# randr — quick random strings

A CLI that hands you a short, varied menu of random strings to pick from. Designed so that no two outputs look like they came from the same person — useful when you want screen names, handles, or labels that can't be visually linked to each other.

## How it works

Run `randr` with no arguments and you get 5 picks, each from a different style/theme category. The structural shape (separator, capitalization, suffix length) is also randomized per call, so two outputs side-by-side don't share a "fingerprint":

```text
 1  uuidv7         019e62ac-2f22-7f93-a169-541bb660b27a
 2  url            cf4yMdy2_6iCoryW
 3  name           LIVELY-BRIDGE-3ba2
 4  constellation  hydra-727
 5  historical     Rembrandt.7bta
```

Categories (one format sampled from each per run): `id` (uuid/uuidv7), `token` (url/api), `memorable` (name/food), `place` (geo/constellation), `character` (character/historical).

Pass a name to get more of one format:

```bash
randr name        # 10 by default
randr geo -n 20
randr uuidv7
```

Even within a single format, each call randomizes separator (`-` / `_` / `.` / none), capitalization (lower / Title / UPPER), and suffix shape (digits / letters / hex / alphanumeric, length 2–6) — so `randr name -n 5` looks like five different people, not one person's pattern.

## Available formats

- **ID/token:** `uuid`, `uuidv7`, `url`, `api`
- **Memorable:** `name`, `food`, `geo`, `constellation`, `character`, `historical`
- **Other (lower-entropy, not in default menu):** `phonetic`, `rhyme`, `music`, `element`, `sports`

## Build

```bash
cargo build --release
./target/release/randr
```
