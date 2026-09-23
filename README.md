# Post-Apollo Zellij

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
