//! Grid structure for managing pre-transformed geographic coordinate grids

use crate::error::{Error, Result};
use crate::marching_squares::{generate_isobands, generate_isolines};
use crate::types::{GridPoint, MarchingSquaresConfig};
use geojson::Feature;

/// A geographic grid with pre-transformed coordinates
///
/// This structure holds a 2D grid of points with geographic coordinates (lon, lat)
/// and associated data values. The coordinates should be pre-transformed to WGS84
/// before creating the grid to avoid expensive per-point transformations during
/// contour generation.
///
/// # Grid Layout
///
/// The grid is stored in row-major order:
/// ```text
/// points[0][0]  points[0][1]  ...  points[0][cols-1]    (top row)
/// points[1][0]  points[1][1]  ...  points[1][cols-1]
/// ...
/// points[rows-1][0]  ...  points[rows-1][cols-1]        (bottom row)
/// ```
#[derive(Debug, Clone)]
pub struct GeoGrid {
    /// Grid data in row-major order [row][col]
    points: Vec<Vec<GridPoint>>,
    /// Number of rows
    rows: usize,
    /// Number of columns
    cols: usize,
    /// Configuration for marching squares algorithm
    config: MarchingSquaresConfig,
}

impl GeoGrid {
    /// Create a new GeoGrid from a 2D array of grid points
    ///
    /// # Arguments
    ///
    /// * `points` - 2D vector of GridPoints in row-major order [row][col]
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The grid is empty
    /// - Rows have inconsistent lengths
    /// - Grid dimensions are less than 2x2
    /// - Any coordinates are invalid (outside valid lat/lon ranges)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use geo_marching_squares_rs::{GeoGrid, GridPoint};
    ///
    /// let points = vec![
    ///     vec![
    ///         GridPoint::new(-100.0, 41.0, 10.0),
    ///         GridPoint::new(-99.0, 41.0, 20.0),
    ///     ],
    ///     vec![
    ///         GridPoint::new(-100.0, 40.0, 15.0),
    ///         GridPoint::new(-99.0, 40.0, 25.0),
    ///     ],
    /// ];
    ///
    /// let grid = GeoGrid::from_points(points)?;
    /// # Ok::<(), geo_marching_squares_rs::Error>(())
    /// ```
    pub fn from_points(points: Vec<Vec<GridPoint>>) -> Result<Self> {
        if points.is_empty() {
            return Err(Error::EmptyGrid);
        }

        let rows = points.len();
        let cols = points[0].len();

        if rows < 2 || cols < 2 {
            return Err(Error::invalid_dimensions(format!(
                "Grid must be at least 2x2, got {}x{}",
                rows, cols
            )));
        }

        // Validate all rows have same length
        for (i, row) in points.iter().enumerate() {
            if row.len() != cols {
                return Err(Error::invalid_dimensions(format!(
                    "Inconsistent row length at row {}: expected {}, got {}",
                    i,
                    cols,
                    row.len()
                )));
            }
        }

        // Validate coordinates
        for row in points.iter() {
            for point in row.iter() {
                if !point.is_valid() {
                    return Err(Error::invalid_coordinates(point.lat, point.lon));
                }
            }
        }

        Ok(Self {
            points,
            rows,
            cols,
            config: MarchingSquaresConfig::default(),
        })
    }

    /// Create a new GeoGrid with custom configuration
    pub fn from_points_with_config(
        points: Vec<Vec<GridPoint>>,
        config: MarchingSquaresConfig,
    ) -> Result<Self> {
        let mut grid = Self::from_points(points)?;
        grid.config = config;
        Ok(grid)
    }

    /// Get the number of rows in the grid
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns in the grid
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get a reference to a specific grid point
    ///
    /// # Arguments
    ///
    /// * `row` - Row index
    /// * `col` - Column index
    ///
    /// # Returns
    ///
    /// Returns `None` if indices are out of bounds
    pub fn get(&self, row: usize, col: usize) -> Option<&GridPoint> {
        self.points.get(row).and_then(|r| r.get(col))
    }

    /// Get a reference to the configuration
    pub fn config(&self) -> &MarchingSquaresConfig {
        &self.config
    }

    /// Get a mutable reference to the configuration
    pub fn config_mut(&mut self) -> &mut MarchingSquaresConfig {
        &mut self.config
    }

    /// Generate isobands (filled contours) for the given thresholds
    ///
    /// Isobands are polygons representing areas where values fall between consecutive thresholds.
    /// For n thresholds, generates n-1 isobands.
    ///
    /// # Arguments
    ///
    /// * `thresholds` - Sorted array of threshold values
    ///
    /// # Returns
    ///
    /// A vector of GeoJSON Features, each representing one isoband with properties:
    /// - `lower_level`: Lower threshold value
    /// - `upper_level`: Upper threshold value
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Fewer than 2 thresholds are provided
    /// - Thresholds are not in ascending order
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use geo_marching_squares_rs::{GeoGrid, GridPoint};
    /// # let grid = GeoGrid::from_points(vec![
    /// #     vec![GridPoint::new(-100.0, 41.0, 10.0), GridPoint::new(-99.0, 41.0, 20.0)],
    /// #     vec![GridPoint::new(-100.0, 40.0, 15.0), GridPoint::new(-99.0, 40.0, 25.0)],
    /// # ])?;
    /// let isobands = grid.isobands(&[10.0, 15.0, 20.0, 25.0])?;
    /// // Returns 3 isobands: [10-15], [15-20], [20-25]
    /// # Ok::<(), geo_marching_squares_rs::Error>(())
    /// ```
    pub fn isobands(&self, thresholds: &[f64]) -> Result<Vec<Feature>> {
        if thresholds.len() < 2 {
            return Err(Error::invalid_thresholds(
                "At least 2 thresholds required for isobands",
            ));
        }

        // Validate thresholds are sorted
        for i in 1..thresholds.len() {
            if thresholds[i] <= thresholds[i - 1] {
                return Err(Error::invalid_thresholds(
                    "Thresholds must be in ascending order",
                ));
            }
        }

        generate_isobands(self, thresholds)
    }

    /// Generate isolines (contour lines) for the given values
    ///
    /// Isolines are lines representing areas where values equal specific levels.
    ///
    /// # Arguments
    ///
    /// * `levels` - Array of contour level values
    ///
    /// # Returns
    ///
    /// A vector of GeoJSON Features, each representing one isoline with property:
    /// - `isovalue`: The contour level value
    ///
    /// # Errors
    ///
    /// Returns an error if no levels are provided
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use geo_marching_squares_rs::{GeoGrid, GridPoint};
    /// # let grid = GeoGrid::from_points(vec![
    /// #     vec![GridPoint::new(-100.0, 41.0, 10.0), GridPoint::new(-99.0, 41.0, 20.0)],
    /// #     vec![GridPoint::new(-100.0, 40.0, 15.0), GridPoint::new(-99.0, 40.0, 25.0)],
    /// # ])?;
    /// let isolines = grid.isolines(&[15.0, 20.0])?;
    /// // Returns 2 isolines at values 15.0 and 20.0
    /// # Ok::<(), geo_marching_squares_rs::Error>(())
    /// ```
    pub fn isolines(&self, levels: &[f64]) -> Result<Vec<Feature>> {
        if levels.is_empty() {
            return Err(Error::invalid_thresholds(
                "At least 1 level required for isolines",
            ));
        }

        generate_isolines(self, levels)
    }

    /// Get an iterator over all grid points
    pub fn iter(&self) -> impl Iterator<Item = &GridPoint> {
        self.points.iter().flat_map(|row| row.iter())
    }

    /// Get a reference to the underlying 2D point array
    pub fn points(&self) -> &Vec<Vec<GridPoint>> {
        &self.points
    }

    /// Get the bounding box of the grid
    ///
    /// Returns (min_lon, min_lat, max_lon, max_lat)
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        self.iter().fold(
            (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            |(min_lon, min_lat, max_lon, max_lat), point| {
                (
                    min_lon.min(point.lon),
                    min_lat.min(point.lat),
                    max_lon.max(point.lon),
                    max_lat.max(point.lat),
                )
            },
        )
    }

    /// Get the value range in the grid
    ///
    /// Returns (min_value, max_value)
    pub fn value_range(&self) -> (f32, f32) {
        self.iter().fold(
            (f32::INFINITY, f32::NEG_INFINITY),
            |(min_val, max_val), point| {
                (min_val.min(point.value), max_val.max(point.value))
            },
        )
    }
}

/// Implement IntoIterator for GeoGrid references
impl<'a> IntoIterator for &'a GeoGrid {
    type Item = &'a GridPoint;
    type IntoIter = std::iter::FlatMap<
        std::slice::Iter<'a, Vec<GridPoint>>,
        std::slice::Iter<'a, GridPoint>,
        fn(&'a Vec<GridPoint>) -> std::slice::Iter<'a, GridPoint>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.points.iter().flat_map(|row| row.iter())
    }
}

/// Implement TryFrom for creating GeoGrid from Vec<Vec<GridPoint>>
impl TryFrom<Vec<Vec<GridPoint>>> for GeoGrid {
    type Error = Error;

    fn try_from(points: Vec<Vec<GridPoint>>) -> Result<Self> {
        Self::from_points(points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_grid() -> Vec<Vec<GridPoint>> {
        vec![
            vec![
                GridPoint::new(-100.0, 41.0, 10.0),
                GridPoint::new(-99.0, 41.0, 20.0),
                GridPoint::new(-98.0, 41.0, 30.0),
            ],
            vec![
                GridPoint::new(-100.0, 40.0, 15.0),
                GridPoint::new(-99.0, 40.0, 25.0),
                GridPoint::new(-98.0, 40.0, 35.0),
            ],
            vec![
                GridPoint::new(-100.0, 39.0, 12.0),
                GridPoint::new(-99.0, 39.0, 22.0),
                GridPoint::new(-98.0, 39.0, 32.0),
            ],
        ]
    }

    #[test]
    fn test_create_grid() {
        let grid = GeoGrid::from_points(create_test_grid()).unwrap();
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 3);
    }

    #[test]
    fn test_empty_grid() {
        let result = GeoGrid::from_points(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_too_small_grid() {
        let points = vec![vec![GridPoint::new(-100.0, 40.0, 10.0)]];
        let result = GeoGrid::from_points(points);
        assert!(result.is_err());
    }

    #[test]
    fn test_inconsistent_rows() {
        let points = vec![
            vec![GridPoint::new(-100.0, 41.0, 10.0), GridPoint::new(-99.0, 41.0, 20.0)],
            vec![GridPoint::new(-100.0, 40.0, 15.0)], // Wrong length
        ];
        let result = GeoGrid::from_points(points);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_coordinates() {
        let points = vec![
            vec![
                GridPoint::new(-100.0, 41.0, 10.0),
                GridPoint::new(-99.0, 91.0, 20.0), // Invalid latitude
            ],
            vec![
                GridPoint::new(-100.0, 40.0, 15.0),
                GridPoint::new(-99.0, 40.0, 25.0),
            ],
        ];
        let result = GeoGrid::from_points(points);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_point() {
        let grid = GeoGrid::from_points(create_test_grid()).unwrap();

        let point = grid.get(0, 0).unwrap();
        assert_eq!(point.lon, -100.0);
        assert_eq!(point.lat, 41.0);
        assert_eq!(point.value, 10.0);

        let point = grid.get(2, 2).unwrap();
        assert_eq!(point.lon, -98.0);
        assert_eq!(point.lat, 39.0);
        assert_eq!(point.value, 32.0);

        assert!(grid.get(10, 10).is_none());
    }

    #[test]
    fn test_bounds() {
        let grid = GeoGrid::from_points(create_test_grid()).unwrap();
        let (min_lon, min_lat, max_lon, max_lat) = grid.bounds();

        assert_eq!(min_lon, -100.0);
        assert_eq!(min_lat, 39.0);
        assert_eq!(max_lon, -98.0);
        assert_eq!(max_lat, 41.0);
    }

    #[test]
    fn test_value_range() {
        let grid = GeoGrid::from_points(create_test_grid()).unwrap();
        let (min_val, max_val) = grid.value_range();

        assert_eq!(min_val, 10.0);
        assert_eq!(max_val, 35.0);
    }
}
