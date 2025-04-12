<div align="center">

# Bevy Ghx Grid

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![bevy_ghx_grid on crates.io](https://img.shields.io/crates/v/bevy_ghx_grid)](https://crates.io/crates/bevy_ghx_grid)
[![bevy_ghx_grid on docs.io](https://docs.rs/bevy_ghx_grid/badge.svg)](https://docs.rs/bevy_ghx_grid)

Uses and exports [`ghx_grid`](https://github.com/Henauxg/ghx_grid), as well as additional plugins and utilities dedicated to [`Bevy`](https://github.com/bevyengine/bevy).

</div>

## Grid debug plugin

`GridDebugPlugin` provides debug utilities for the grid-types within `ghx_grid`:
  - Can draw a debug view of any 2d/3d grid
  - Can draw debug markers on any cells of a grid (controlled via bevy events)

Use it by inserting a `DebugGridView3d` bundle on your `Grid` entity (or `DebugGridView2d`, depending on your Bevy Camera).

<p align="center">
  <img alt="debug_grid_and_markers" src="docs/assets/debug_grid_and_markers.png" width="60%">
</p>

## Cargo features

*Find the list and description in [Cargo.toml](Cargo.toml)*

- `debug-plugin` *[default]*: compiles the grid debug plugin and its systems.
- `reflect` *[default]*: enables `ghx_grid` reflect feature.

*See also the [main crate](https://github.com/Henauxg/ghx_grid#cargo-features) cargo features*

## Compatible Bevy versions

| **bevy_ghx_grid** | **ghx_grid** | **bevy** |
| :---------------- | :----------- | :------- |
| 0.6               | 0.6          | 0.16     |
| 0.5               | 0.5          | 0.15     |
| 0.4               | 0.4          | 0.14     |
| 0.3               | 0.3          | 0.13     |
| 0.2               | 0.2          | 0.13     |
| 0.1               | 0.1          | 0.12     |

## License

### Code

bevy_ghx_grid is free and open source. All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
