//!
//! Handy things for 2d grids.
//!
#![feature(step_trait)]

pub mod auto_index_impl;
pub mod bounded_grid;
mod cache_map;
pub mod infinite_grid;
pub mod slice;

pub use bounded_grid::Grid;
pub use infinite_grid::InfiniteGrid;
