#![warn(missing_docs)]

//! This library encapsulates (and re-exports) the "ghx_grid" library for 2D & 3D grids.
//! It also provides additional utilities to manipulate & debug 2d & 3d grid data with Bevy.
pub use ghx_grid;

/// Utilities & debug tools/plugins for manipulating grids
#[cfg(feature = "debug-plugin")]
pub mod debug_plugin;
