✦︎✦︎✦︎ Meta Apollo Logos //

# 🖳 POST-APOLLO // ZELLIJ

![](BUILD/assets/design/chassis/focus-rail.svg)

> **STATE //** active \~\~ **VIEW //** terminal multiplexer / receiver control layer

> **Post-Apollo Zellij contains the live multiplexer configuration, layouts, and custom receiver-style WASM plugin.**

### 🧭 MAP // REPOSITORY

![](BUILD/assets/design/chassis/nav-rail.svg)

// [🧭 ATLAS](./ATLAS/) \~\~ // [✮˙๋࣭⭑ MODEL](./MODEL/) \~\~ // [🖨 BUILD](./BUILD/) \~\~ // [⚒ DEV](./DEV/) \~\~ // [🖳 OPERATE](./OPERATE/) \~\~ // [⊹ ࣪ℼ˖ EVIDENCE](./EVIDENCE/) \~\~ // [࣪⋅˚🕮‧₊˚ ARCHIVE](./ARCHIVE/)

---

### ★⋆˙ CORE // RUNTIME LAYOUT

The live `config.kdl`, layouts, and plugin source stay in their existing paths. The seven rooms are the documentation and semantic layer around them.

---

Post-Apollo Zellij configuration, layout, and custom WASM plugin.

## Current environment

Zellij:

    0.45.0

Rust WASM target:

    wasm32-wasip1

## Contents

- `config.kdl` — live Zellij configuration
- `layouts/post-apollo.kdl` — Post-Apollo layout
- `post-apollo/plugin/` — custom Post-Apollo Zellij plugin source
- `post-apollo/plugin/post-apollo.kdl` — plugin-related configuration

## Plugin

Rust package:

    post-apollo

Version:

    0.1.0

Zellij tile API:

    zellij-tile = "0.45.0"

Build output under `target/` is intentionally not versioned.

The generated WASM artifact can be rebuilt from source.
