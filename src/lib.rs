//! # geo-marching-squares-rs
//!
//! A high-performance Rust implementation of the marching squares algorithm designed specifically
//! for geographic data (lat/lon coordinates). This crate is a direct port of a proven Java
//! implementation, addressing fundamental performance bottlenecks in existing contour libraries.
//!
//! ## Key Features
//!
//! - **Full 81-case implementation**: Direct port from proven Java implementation (2036 lines)
//! - **Complete shape types**: Triangle, Pentagon, Rectangle, Trapezoid, Hexagon, Saddle, Square
//! - **Isolines & Isobands**: 16-case isolines and 81-case isobands with edge tracing
//! - **Pre-transformed coordinates**: Eliminates expensive per-point coordinate transformations
//! - **Cosine interpolation**: Handles Earth curvature effects with proven accuracy
//! - **Polygon nesting & holes**: Automatic interior ring detection for complex topologies
//! - **Parallel processing**: Optional rayon-based parallelization for large grids
//! - **GeoJSON output**: RFC 7946 compliant geographic features
//! - **Production tested**: 34 comprehensive tests, all passing
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use geo_marching_squares_rs::{GeoGrid, GridPoint};
//! use geojson::Feature;
//!
//! // Create a grid with pre-transformed coordinates
//! let points = vec![
//!     vec![
//!         GridPoint { lon: -100.0, lat: 40.0, value: 10.0 },
//!         GridPoint { lon: -99.0, lat: 40.0, value: 20.0 },
//!     ],
//!     vec![
//!         GridPoint { lon: -100.0, lat: 41.0, value: 15.0 },
//!         GridPoint { lon: -99.0, lat: 41.0, value: 25.0 },
//!     ],
//! ];
//!
//! let grid = GeoGrid::from_points(points)?;
//!
//! // Generate isobands (filled contours)
//! let isobands = grid.isobands(&[12.0, 18.0, 22.0])?;
//!
//! // Generate isolines (contour lines)
//! let isolines = grid.isolines(&[15.0, 20.0])?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod cell_shapes;
mod edge_tracing;
mod error;
mod grid;
mod marching_squares;
mod polygon_util;
mod simd_ops;
mod types;

pub mod interpolation;

pub use error::{Error, Result};
pub use grid::GeoGrid;
pub use types::{Edge, GridPoint, InterpolationMethod, MarchingSquaresConfig, Move, Point, Side};

// Re-export commonly used types
pub use geojson::{Feature, FeatureCollection};
