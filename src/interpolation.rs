//! Interpolation functions for marching squares algorithm
//!
//! This module provides cosine interpolation with center bias, ported from the proven
//! Java implementation. The interpolation handles smooth contour generation in geographic
//! space without requiring expensive great circle calculations.

use crate::types::{Point, Side};
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
pub fn interpolate_point(
    level: f64,
    value0: f64,
    value1: f64,
    point0: &Point,
    point1: &Point,
    smoothing_factor: f64,
) -> Point {
    // Linear interpolation factor
    let mu = (level - value0) / (value1 - value0);

    // Apply cosine smoothing
    let mu2 = (1.0 - (mu * PI).cos()) / 2.0;

    // Apply center bias with smoothing factor
    let center_diff = (mu2 - 0.5) * smoothing_factor;
    let new_mu = 0.5 + center_diff;

    // Linear interpolation with adjusted mu
    let x = (1.0 - new_mu) * point0.x + new_mu * point1.x;
    let y = (1.0 - new_mu) * point0.y + new_mu * point1.y;

    Point::new(x, y)
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

#[cfg(feature = "great-circle")]
/// Interpolates using spherical (great circle) calculations.
///
/// This is more accurate for very large distances but also more expensive.
/// Requires the `great-circle` feature.
pub fn interpolate_point_spherical(
    level: f64,
    value0: f64,
    value1: f64,
    point0: &Point,
    point1: &Point,
    smoothing_factor: f64,
) -> Point {
    use geo_types::{Coord, Point as GeoPoint};

    // Linear interpolation factor
    let mu = (level - value0) / (value1 - value0);

    // Apply cosine smoothing
    let mu2 = (1.0 - (mu * PI).cos()) / 2.0;

    // Apply center bias with smoothing factor
    let center_diff = (mu2 - 0.5) * smoothing_factor;
    let new_mu = 0.5 + center_diff;

    // Spherical interpolation
    let p0 = Coord { x: point0.x, y: point0.y };
    let p1 = Coord { x: point1.x, y: point1.y };

    // Convert to radians
    let lon0 = point0.x.to_radians();
    let lat0 = point0.y.to_radians();
    let lon1 = point1.x.to_radians();
    let lat1 = point1.y.to_radians();

    // Calculate great circle distance
    let d = (lat0.sin() * lat1.sin() +
             lat0.cos() * lat1.cos() * (lon1 - lon0).cos()).acos();

    // Interpolate along great circle
    let a = ((1.0 - new_mu) * d).sin() / d.sin();
    let b = (new_mu * d).sin() / d.sin();

    let x = a * lat0.cos() * lon0.cos() + b * lat1.cos() * lon1.cos();
    let y = a * lat0.cos() * lon0.sin() + b * lat1.cos() * lon1.sin();
    let z = a * lat0.sin() + b * lat1.sin();

    let lat = z.atan2((x * x + y * y).sqrt());
    let lon = y.atan2(x);

    Point::new(lon.to_degrees(), lat.to_degrees())
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
}
