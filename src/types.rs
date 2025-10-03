//! Core data types for geographic marching squares

use serde::{Deserialize, Serialize};

/// Round coordinate to 6 decimal places (~111mm precision at equator)
/// This helps edge tracing by ensuring consistent coordinate values
fn round_coordinate(coord: f64) -> f64 {
    (coord * 1_000_000.0).round() / 1_000_000.0
}

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
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a point from longitude and latitude with coordinate rounding
    /// Rounds to 6 decimal places (~111mm precision at equator)
    pub fn from_lon_lat(lon: f64, lat: f64) -> Self {
        Self {
            x: round_coordinate(lon),
            y: round_coordinate(lat),
        }
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
#[non_exhaustive]
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
    pub const fn new(start: Point, end: Point, move_dir: Move) -> Self {
        Self {
            start,
            end,
            move_dir,
        }
    }
}

/// Smoothing factor for interpolation (0.0 to 1.0)
///
/// This newtype ensures that smoothing factors are within the valid range.
/// Typical values are close to 1.0 (e.g., 0.999) for smooth contours.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SmoothingFactor(f64);

impl SmoothingFactor {
    /// Create a new smoothing factor, clamping to valid range [0.0, 1.0]
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Get the raw value
    pub fn get(self) -> f64 {
        self.0
    }
}

impl Default for SmoothingFactor {
    fn default() -> Self {
        Self(0.999)
    }
}

impl From<f64> for SmoothingFactor {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<SmoothingFactor> for f64 {
    fn from(sf: SmoothingFactor) -> f64 {
        sf.0
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
    pub smoothing_factor: SmoothingFactor,
}

impl Default for MarchingSquaresConfig {
    fn default() -> Self {
        Self {
            use_parallel: cfg!(feature = "parallel"),
            interpolation_method: InterpolationMethod::Cosine,
            smoothing_factor: SmoothingFactor::default(),
        }
    }
}

impl MarchingSquaresConfig {
    /// Create a new config builder with default settings
    pub fn builder() -> MarchingSquaresConfigBuilder {
        MarchingSquaresConfigBuilder::default()
    }

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

/// Builder for MarchingSquaresConfig with fluent API
#[derive(Debug, Default)]
pub struct MarchingSquaresConfigBuilder {
    use_parallel: Option<bool>,
    interpolation_method: Option<InterpolationMethod>,
    smoothing_factor: Option<SmoothingFactor>,
}

impl MarchingSquaresConfigBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to use parallel processing
    pub fn with_parallel(mut self, enabled: bool) -> Self {
        self.use_parallel = Some(enabled);
        self
    }

    /// Set the interpolation method
    pub fn with_interpolation(mut self, method: InterpolationMethod) -> Self {
        self.interpolation_method = Some(method);
        self
    }

    /// Set the smoothing factor
    pub fn with_smoothing(mut self, factor: impl Into<SmoothingFactor>) -> Self {
        self.smoothing_factor = Some(factor.into());
        self
    }

    /// Build the configuration
    pub fn build(self) -> MarchingSquaresConfig {
        let defaults = MarchingSquaresConfig::default();
        MarchingSquaresConfig {
            use_parallel: self.use_parallel.unwrap_or(defaults.use_parallel),
            interpolation_method: self.interpolation_method.unwrap_or(defaults.interpolation_method),
            smoothing_factor: self.smoothing_factor.unwrap_or(defaults.smoothing_factor),
        }
    }
}