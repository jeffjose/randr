# randr — quick random strings

A CLI that gives you grids of random strings to pick from. **Within a section** the style is uniform (same separator/case/tag shape) so you can scan candidates of one "feel" — **across sections** the format and style change, so different sections look like they came from different people.

## How it works

Run `randr` with no arguments and you get 5 themed sections, each laid out as a grid sized to your terminal:

```text
id (uuidv7)
  019e62bd-5713-7043-99c1-9582e2eba8d2  019e62bd-5713-7043-99c1-9599435e19df
  019e62bd-5713-7043-99c1-95a8e389f194  019e62bd-5713-7043-99c1-95bcec49ab6d
  019e62bd-5713-7043-99c1-95c651f0ffe2  019e62bd-5713-7043-99c1-95d5aed8edb2

token (api)
  7U6W8FI6BVQIJIV84N7WAERB  9LKD3ISEPC08JSKT523Q4TFT  6GB9SYJXTN6MY6TTP6ZNYLTO
  UX2XWRCXC85Q88N3PYHK5K0Z  BH84DYT5ADDWXG3IVT1L59FY  7GUTMHNU77MC72N8B7YJIETG

memorable (name)
  hrew.PeacefulPlanet  rdjj.HumbleBridge   wsly.PeacefulUniverse
  xulf.GentlePhoenix   zrcw.EagerElephant  nctl.RunningMountain

place (geo)
  SYDNEY-76706  THAMES-11650  GANGES-64597  CAIRO-22297
  ANDES-97411   TOKYO-53700   SANTORINI-32492  ALPS-37916

character (character)
  W00DY.2568  W0NDERW0MAN.1793  B0W53R.904  IRONMAN.8341
  BATMAN.7    KENOB1.288        81180.3750  S4UR0N.6
```

Each row of the output is a different theme group. Within a group, every candidate uses the same locked style — only the words differ. Rerun to roll a new format/style per section.

Themes and the formats they sample from:

- **id**: uuid, uuidv7
- **token**: url, api
- **memorable**: name (adjective+noun), food
- **place**: geo, constellation
- **character**: character, historical

Pass a name to focus on one format — it generates many candidates, all in one locked style (rerun for a different style):

```bash
randr name
randr food
randr geo
randr name -n 40
```

## Style axes (rolled once per section / per `randr <format>` call)

- separator: `-` / `_` / `.` / none
- case: lower / Title / UPPER
- tag position: prefix / between words / suffix
- tag shape: small int / digits / letters / hex / alphanumeric, length 2–6
- mixed body/tag separators (e.g. `Bold_garden42`)
- leetspeak: ~1-in-4 sections leetify (`a→4`, `e→3`, `o→0`, …)

## Available formats

- ID/token: `uuid`, `uuidv7`, `url`, `api`
- Memorable: `name`, `food`, `geo`, `constellation`, `character`, `historical`
- Other (lower-entropy, not in default menu): `phonetic`, `rhyme`, `music`, `element`, `sports`

## Build

```bash
cargo build --release
./target/release/randr
```
