//! Core data types for geographic marching squares

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Round coordinate to 5 decimal places (~1.1 meter precision at equator)
/// This matches the Java implementation (positionAccuracy = 5)
///
/// Uses Rust's built-in `round()` which implements "round half-way cases away from 0.0"
/// This is equivalent to Java's BigDecimal.setScale(5, RoundingMode.HALF_UP)
///
/// IMPORTANT: Only applied at final GeoJSON output, NOT during interpolation.
/// This ensures adjacent cells compute identical edge endpoints during tracing.
pub fn round_coordinate(coord: f64) -> f64 {
    // Rust's round() already does HALF_UP (rounds 0.5 away from zero)
    // which matches Java's RoundingMode.HALF_UP behavior
    (coord * 100_000.0).round() / 100_000.0
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

/// A point that can be either actual (with coordinates) or placeholder (to be interpolated)
///
/// Java: Shape.java Point class
/// This matches the Java implementation where Points can be:
/// - Actual: x and y are set (for corners that fall within the band)
/// - Placeholder: x and y are null, with value/limit/side set (to be interpolated later)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    /// X coordinate (longitude for geographic data) - None for placeholder points
    pub x: Option<f64>,
    /// Y coordinate (latitude for geographic data) - None for placeholder points
    pub y: Option<f64>,
    /// Value at this point (for placeholder points awaiting interpolation)
    pub value: Option<f64>,
    /// Threshold limit (upper or lower) for interpolation
    pub limit: Option<f64>,
    /// Which side of the cell this point is on (for interpolation)
    pub side: Option<Side>,
}

impl Point {
    /// Create an actual point with coordinates set
    /// Java: Shape.java:182-185 - new Point(coords.getLongitude(), coords.getLatitude())
    pub const fn actual(x: f64, y: f64) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            value: None,
            limit: None,
            side: None,
        }
    }

    /// Create a placeholder point (to be interpolated later)
    /// Java: Shape.java:229-236 - creates Points with x=null, y=null
    pub const fn placeholder(value: f64, limit: f64, side: Side) -> Self {
        Self {
            x: None,
            y: None,
            value: Some(value),
            limit: Some(limit),
            side: Some(side),
        }
    }

    /// Create a new point (legacy compatibility - creates actual point)
    pub const fn new(x: f64, y: f64) -> Self {
        Self::actual(x, y)
    }

    /// Create a point from longitude and latitude (creates actual point)
    pub fn from_lon_lat(lon: f64, lat: f64) -> Self {
        Self::actual(lon, lat)
    }

    /// Get longitude (returns None for placeholder points)
    pub fn lon(&self) -> Option<f64> {
        self.x
    }

    /// Get latitude (returns None for placeholder points)
    pub fn lat(&self) -> Option<f64> {
        self.y
    }

    /// Check if this is a placeholder point (needs interpolation)
    pub fn is_placeholder(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    /// Check if this is an actual point (has coordinates)
    pub fn is_actual(&self) -> bool {
        self.x.is_some() && self.y.is_some()
    }
}

impl From<GridPoint> for Point {
    fn from(grid_point: GridPoint) -> Self {
        Self::actual(grid_point.lon, grid_point.lat)
    }
}

// Implement Hash and Eq for Point to enable HashMap usage
// Java: Point.java equals() and hashCode() compare ALL fields
// This is CRITICAL for deduplication and HashMap lookups
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash all fields - matches Java Point.hashCode()
        self.x.map(|v| v.to_bits()).hash(state);
        self.y.map(|v| v.to_bits()).hash(state);
        self.value.map(|v| v.to_bits()).hash(state);
        self.limit.map(|v| v.to_bits()).hash(state);
        self.side.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        // Compare ALL fields - matches Java Point.equals()
        // CRITICAL: Not just x,y! Must include value, limit, side
        self.x == other.x
            && self.y == other.y
            && self.value == other.value
            && self.limit == other.limit
            && self.side == other.side
    }
}

impl Eq for Point {}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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