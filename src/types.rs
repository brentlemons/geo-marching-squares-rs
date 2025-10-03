//! Core data types for geographic marching squares

use serde::{Deserialize, Serialize};

/// A point with geographic coordinates and a data value
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GridPoint {
    /// Longitude in degrees (WGS84)
    pub lon: f64,
    /// Latitude in degrees (WGS84)
    pub lat: f64,
    /// Data value at this point
    pub value: f32,
}

impl GridPoint {
    /// Create a new grid point
    pub fn new(lon: f64, lat: f64, value: f32) -> Self {
        Self { lon, lat, value }
    }

    /// Validate that coordinates are within reasonable bounds
    pub fn is_valid(&self) -> bool {
        self.lat >= -90.0 && self.lat <= 90.0 && self.lon >= -180.0 && self.lon <= 180.0
    }
}

/// A simple 2D point for geometric calculations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    /// X coordinate (longitude for geographic data)
    pub x: f64,
    /// Y coordinate (latitude for geographic data)
    pub y: f64,
}

impl Point {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a point from longitude and latitude
    pub fn from_lon_lat(lon: f64, lat: f64) -> Self {
        Self { x: lon, y: lat }
    }

    /// Get longitude (assuming this point represents geographic coordinates)
    pub fn lon(&self) -> f64 {
        self.x
    }

    /// Get latitude (assuming this point represents geographic coordinates)
    pub fn lat(&self) -> f64 {
        self.y
    }
}

impl From<GridPoint> for Point {
    fn from(grid_point: GridPoint) -> Self {
        Self {
            x: grid_point.lon,
            y: grid_point.lat,
        }
    }
}

/// Interpolation method for contour generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InterpolationMethod {
    /// Cosine interpolation with center bias (default, fast and accurate for typical grids)
    Cosine,
    /// Great circle (spherical) interpolation (more accurate for large distances, slower)
    GreatCircle,
}

impl Default for InterpolationMethod {
    fn default() -> Self {
        Self::Cosine
    }
}

/// Represents a side of a grid cell for marching squares algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

/// Direction of movement when following edges across cells
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Right,
    Down,
    Left,
    Up,
    None, // Edge doesn't cross cell boundary
}

impl Move {
    /// Apply this move to a cell position, returning the new (row, col)
    pub fn apply(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        match self {
            Move::Right => Some((row, col + 1)),
            Move::Down => Some((row + 1, col)),
            Move::Left => col.checked_sub(1).map(|c| (row, c)),
            Move::Up => row.checked_sub(1).map(|r| (r, col)),
            Move::None => None,
        }
    }
}

/// An edge in the marching squares algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    /// Starting point of the edge
    pub start: Point,
    /// Ending point of the edge
    pub end: Point,
    /// Direction to move to the next cell
    pub move_dir: Move,
}

impl Edge {
    /// Create a new edge
    pub fn new(start: Point, end: Point, move_dir: Move) -> Self {
        Self {
            start,
            end,
            move_dir,
        }
    }
}

/// Configuration for marching squares algorithm behavior
#[derive(Debug, Clone)]
pub struct MarchingSquaresConfig {
    /// Whether to use parallel processing (requires 'parallel' feature)
    pub use_parallel: bool,
    /// Interpolation method to use
    pub interpolation_method: InterpolationMethod,
    /// Smoothing factor for interpolation (0.0 to 1.0, typically 0.999)
    pub smoothing_factor: f64,
}

impl Default for MarchingSquaresConfig {
    fn default() -> Self {
        Self {
            use_parallel: cfg!(feature = "parallel"),
            interpolation_method: InterpolationMethod::Cosine,
            smoothing_factor: 0.999,
        }
    }
}

impl MarchingSquaresConfig {
    /// Create a new config with great circle interpolation
    ///
    /// Note: Great circle interpolation is more accurate for large distances
    /// but significantly slower. Use only when grid spacing is very large
    /// (>100km) or for polar regions.
    pub fn with_great_circle() -> Self {
        Self {
            interpolation_method: InterpolationMethod::GreatCircle,
            ..Default::default()
        }
    }

    /// Create a new config with cosine interpolation (default)
    pub fn with_cosine() -> Self {
        Self::default()
    }
}