//! Interpolation functions for marching squares algorithm
//!
//! This module provides multiple interpolation methods:
//! - **Cosine interpolation** (default): Fast and accurate for typical grid spacings (3-10km)
//! - **Great circle interpolation**: More accurate for large distances or polar regions
//!
//! The cosine method is ported from the proven Java implementation.

use crate::types::{InterpolationMethod, Point, Side};
use std::f64::consts::PI;

/// Interpolates a point along a cell edge using cosine interpolation with center bias.
///
/// This implementation follows the proven Java algorithm:
/// ```text
/// mu = (level - value0) / (value1 - value0)
/// mu2 = (1.0 - cos(mu * PI)) / 2.0         // Cosine smoothing
/// centerDiff = (mu2 - 0.5) * 0.999         // Center bias (smoothing factor)
/// newMu = 0.5 + centerDiff
/// x = (1.0 - newMu) * point0.x + newMu * point1.x
/// y = (1.0 - newMu) * point0.y + newMu * point1.y
/// ```
///
/// # Arguments
///
/// * `level` - The contour level to interpolate to
/// * `value0` - The data value at the first point
/// * `value1` - The data value at the second point
/// * `point0` - The first point (lon, lat)
/// * `point1` - The second point (lon, lat)
/// * `smoothing_factor` - Smoothing factor (default 0.999)
///
/// # Returns
///
/// The interpolated point between point0 and point1 at the given level
///
/// # Example
///
/// ```
/// use geo_marching_squares_rs::interpolation::interpolate_point;
/// use geo_marching_squares_rs::Point;
///
/// let p0 = Point::new(-100.0, 40.0);
/// let p1 = Point::new(-99.0, 40.0);
/// let result = interpolate_point(15.0, 10.0, 20.0, &p0, &p1, 0.999);
/// // Result will be approximately halfway between p0 and p1
/// ```
#[inline]
pub fn interpolate_point(
    level: f64,
    value0: f64,
    value1: f64,
    point0: &Point,
    point1: &Point,
    smoothing_factor: f64,
) -> Point {
    // Handle degenerate case where value0 == value1
    let value_diff = value1 - value0;
    if value_diff.abs() < 1e-10 {
        // No gradient - return rounded midpoint
        return Point::from_lon_lat(
            crate::types::round_coordinate((point0.x + point1.x) / 2.0),
            crate::types::round_coordinate((point0.y + point1.y) / 2.0),
        );
    }

    // Linear interpolation factor
    let mu = (level - value0) / value_diff;

    // Apply cosine smoothing
    let mu2 = (1.0 - (mu * PI).cos()) / 2.0;

    // Apply center bias with smoothing factor
    let center_diff = (mu2 - 0.5) * smoothing_factor;
    let new_mu = 0.5 + center_diff;

    // Linear interpolation with adjusted mu
    let x = (1.0 - new_mu) * point0.x + new_mu * point1.x;
    let y = (1.0 - new_mu) * point0.y + new_mu * point1.y;

    // Round coordinates for consistency in edge tracing
    // This ensures adjacent cells create identical edge endpoints
    Point::from_lon_lat(
        crate::types::round_coordinate(x),
        crate::types::round_coordinate(y)
    )
}

/// Interpolates a point along a specific side of a grid cell.
///
/// # Arguments
///
/// * `level` - The contour level to interpolate to
/// * `side` - Which side of the cell to interpolate along
/// * `top_left` - Top-left corner point and value
/// * `top_right` - Top-right corner point and value
/// * `bottom_right` - Bottom-right corner point and value
/// * `bottom_left` - Bottom-left corner point and value
/// * `smoothing_factor` - Smoothing factor (default 0.999)
///
/// # Returns
///
/// The interpolated point along the specified cell edge
#[inline]
pub fn interpolate_side(
    level: f64,
    side: Side,
    top_left: (&Point, f64),
    top_right: (&Point, f64),
    bottom_right: (&Point, f64),
    bottom_left: (&Point, f64),
    smoothing_factor: f64,
) -> Point {
    match side {
        Side::Top => interpolate_point(
            level,
            top_left.1,
            top_right.1,
            top_left.0,
            top_right.0,
            smoothing_factor,
        ),
        Side::Right => interpolate_point(
            level,
            top_right.1,
            bottom_right.1,
            top_right.0,
            bottom_right.0,
            smoothing_factor,
        ),
        Side::Bottom => interpolate_point(
            level,
            bottom_left.1,
            bottom_right.1,
            bottom_left.0,
            bottom_right.0,
            smoothing_factor,
        ),
        Side::Left => interpolate_point(
            level,
            top_left.1,
            bottom_left.1,
            top_left.0,
            bottom_left.0,
            smoothing_factor,
        ),
    }
}

/// Dispatches to the appropriate interpolation method
///
/// This is the main entry point for interpolation. It selects between
/// cosine and great circle interpolation based on the method parameter.
#[inline]
pub fn interpolate_with_method(
    method: InterpolationMethod,
    level: f64,
    value0: f64,
    value1: f64,
    point0: &Point,
    point1: &Point,
    smoothing_factor: f64,
) -> Point {
    match method {
        InterpolationMethod::Cosine => {
            interpolate_point(level, value0, value1, point0, point1, smoothing_factor)
        }
        InterpolationMethod::GreatCircle => {
            interpolate_point_great_circle(level, value0, value1, point0, point1, smoothing_factor)
        }
    }
}

/// Interpolates using spherical (great circle) calculations.
///
/// This is more accurate for very large distances but also more expensive.
/// Use only when grid spacing is very large (>100km) or for polar regions.
#[inline]
pub fn interpolate_point_great_circle(
    level: f64,
    value0: f64,
    value1: f64,
    point0: &Point,
    point1: &Point,
    smoothing_factor: f64,
) -> Point {
    // Handle degenerate case where value0 == value1
    let value_diff = value1 - value0;
    if value_diff.abs() < 1e-10 {
        // No gradient - return rounded midpoint
        return Point::from_lon_lat(
            crate::types::round_coordinate((point0.x + point1.x) / 2.0),
            crate::types::round_coordinate((point0.y + point1.y) / 2.0),
        );
    }

    // Linear interpolation factor
    let mu = (level - value0) / value_diff;

    // Apply cosine smoothing
    let mu2 = (1.0 - (mu * PI).cos()) / 2.0;

    // Apply center bias with smoothing factor
    let center_diff = (mu2 - 0.5) * smoothing_factor;
    let new_mu = 0.5 + center_diff;

    // Convert to radians for spherical interpolation
    let lon0 = point0.x.to_radians();
    let lat0 = point0.y.to_radians();
    let lon1 = point1.x.to_radians();
    let lat1 = point1.y.to_radians();

    // Calculate great circle distance
    let d = (lat0.sin() * lat1.sin() +
             lat0.cos() * lat1.cos() * (lon1 - lon0).cos()).acos();

    // Handle degenerate case where points are same or antipodal
    if d.abs() < 1e-10 || (d - PI).abs() < 1e-10 {
        // Points are too close or antipodal - fall back to linear interpolation
        let x = (1.0 - new_mu) * point0.x + new_mu * point1.x;
        let y = (1.0 - new_mu) * point0.y + new_mu * point1.y;
        return Point::from_lon_lat(
            crate::types::round_coordinate(x),
            crate::types::round_coordinate(y)
        );
    }

    // Interpolate along great circle
    let a = ((1.0 - new_mu) * d).sin() / d.sin();
    let b = (new_mu * d).sin() / d.sin();

    let x = a * lat0.cos() * lon0.cos() + b * lat1.cos() * lon1.cos();
    let y = a * lat0.cos() * lon0.sin() + b * lat1.cos() * lon1.sin();
    let z = a * lat0.sin() + b * lat1.sin();

    let lat = z.atan2((x * x + y * y).sqrt());
    let lon = y.atan2(x);

    // Round coordinates for consistency in edge tracing
    // This ensures adjacent cells create identical edge endpoints
    Point::from_lon_lat(
        crate::types::round_coordinate(lon.to_degrees()),
        crate::types::round_coordinate(lat.to_degrees())
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_point_midpoint() {
        let p0 = Point::new(-100.0, 40.0);
        let p1 = Point::new(-99.0, 41.0);

        // Interpolate at the exact midpoint value
        let result = interpolate_point(15.0, 10.0, 20.0, &p0, &p1, 0.999);

        // With center bias, should be very close to midpoint
        assert!((result.x - (-99.5)).abs() < 0.01);
        assert!((result.y - 40.5).abs() < 0.01);
    }

    #[test]
    fn test_interpolate_point_endpoints() {
        let p0 = Point::new(-100.0, 40.0);
        let p1 = Point::new(-99.0, 41.0);

        // At the lower endpoint
        let result = interpolate_point(10.0, 10.0, 20.0, &p0, &p1, 0.999);
        assert!((result.x - p0.x).abs() < 0.5);
        assert!((result.y - p0.y).abs() < 0.5);

        // At the upper endpoint
        let result = interpolate_point(20.0, 10.0, 20.0, &p0, &p1, 0.999);
        assert!((result.x - p1.x).abs() < 0.5);
        assert!((result.y - p1.y).abs() < 0.5);
    }

    #[test]
    fn test_interpolate_side() {
        let tl = Point::new(-100.0, 41.0);
        let tr = Point::new(-99.0, 41.0);
        let br = Point::new(-99.0, 40.0);
        let bl = Point::new(-100.0, 40.0);

        // Interpolate on top edge
        let result = interpolate_side(
            15.0,
            Side::Top,
            (&tl, 10.0),
            (&tr, 20.0),
            (&br, 20.0),
            (&bl, 10.0),
            0.999,
        );

        // Should be on the top edge (y = 41.0) between tl and tr
        assert!((result.y - 41.0).abs() < 0.01);
        assert!(result.x > -100.0 && result.x < -99.0);
    }

    #[test]
    fn test_interpolate_great_circle() {
        let p0 = Point::new(-100.0, 40.0);
        let p1 = Point::new(-99.0, 40.0);

        // Interpolate at midpoint
        let result = interpolate_point_great_circle(15.0, 10.0, 20.0, &p0, &p1, 0.999);

        // Should be close to midpoint (great circle and linear are similar for small distances)
        assert!((result.x - (-99.5)).abs() < 0.1);
        assert!((result.y - 40.0).abs() < 0.1);
    }

    #[test]
    fn test_interpolate_with_method_cosine() {
        let p0 = Point::new(-100.0, 40.0);
        let p1 = Point::new(-99.0, 40.0);

        let result = interpolate_with_method(
            InterpolationMethod::Cosine,
            15.0, 10.0, 20.0, &p0, &p1, 0.999
        );

        // Should match direct cosine interpolation
        let direct = interpolate_point(15.0, 10.0, 20.0, &p0, &p1, 0.999);
        assert!((result.x - direct.x).abs() < 1e-10);
        assert!((result.y - direct.y).abs() < 1e-10);
    }

    #[test]
    fn test_interpolate_with_method_great_circle() {
        let p0 = Point::new(-100.0, 40.0);
        let p1 = Point::new(-99.0, 40.0);

        let result = interpolate_with_method(
            InterpolationMethod::GreatCircle,
            15.0, 10.0, 20.0, &p0, &p1, 0.999
        );

        // Should match direct great circle interpolation
        let direct = interpolate_point_great_circle(15.0, 10.0, 20.0, &p0, &p1, 0.999);
        assert!((result.x - direct.x).abs() < 1e-10);
        assert!((result.y - direct.y).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_vs_great_circle_small_distance() {
        // For small distances (typical grid spacing), both should be very similar
        let p0 = Point::new(-100.0, 40.0);
        let p1 = Point::new(-99.9, 40.0); // 0.1 degree ~= 11km at this latitude

        let cosine_result = interpolate_point(15.0, 10.0, 20.0, &p0, &p1, 0.999);
        let gc_result = interpolate_point_great_circle(15.0, 10.0, 20.0, &p0, &p1, 0.999);

        // Difference should be less than 1 meter for small distances
        let diff_x = (cosine_result.x - gc_result.x).abs();
        let diff_y = (cosine_result.y - gc_result.y).abs();

        assert!(diff_x < 0.0001); // Less than ~10m
        assert!(diff_y < 0.0001);
    }
}
