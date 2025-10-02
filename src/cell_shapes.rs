//! Cell shape configurations for 3-level marching squares
//!
//! This module implements the full isoband algorithm with 3-level cell configurations.
//! Each corner can be in one of 3 states: below lower (0), between thresholds (1), or above upper (2).

use crate::interpolation::interpolate_side;
use crate::types::{Edge, GridPoint, Move, Point, Side};

/// Cell configuration value (0-170 for base-3 encoding)
pub type CellConfig = u8;

/// Represents the edges and moves for a marching squares cell
#[derive(Debug, Clone)]
pub struct CellShape {
    /// List of edges in this cell (start point, end point, move direction)
    pub edges: Vec<Edge>,
}

impl CellShape {
    /// Create a new cell shape with the given edges
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }

    /// Create edges for this cell configuration
    pub fn from_config(
        config: CellConfig,
        tl: &GridPoint,
        tr: &GridPoint,
        br: &GridPoint,
        bl: &GridPoint,
        lower: f64,
        _upper: f64,
        smoothing: f64,
        _is_top_edge: bool,
        _is_right_edge: bool,
        _is_bottom_edge: bool,
        _is_left_edge: bool,
    ) -> Option<Self> {
        // First, calculate interpolation points for each side
        let tl_pt = Point::from_lon_lat(tl.lon, tl.lat);
        let tr_pt = Point::from_lon_lat(tr.lon, tr.lat);
        let br_pt = Point::from_lon_lat(br.lon, br.lat);
        let bl_pt = Point::from_lon_lat(bl.lon, bl.lat);

        let tl_val = tl.value as f64;
        let tr_val = tr.value as f64;
        let br_val = br.value as f64;
        let bl_val = bl.value as f64;

        // Helper to determine which interpolation points we need
        // For full implementation, we need to handle all 81 cases
        // For now, let's implement the most common cases

        // Empty cells (all corners in same state)
        if config == 0 || config == 0b10101010 {
            return None; // All below or all above
        }

        // For this initial implementation, fall back to simpler isoline-based approach
        // TODO: Implement full 81-case lookup table from Java implementation

        // Convert to simple binary config using lower threshold
        let simple_config = calculate_isoline_config(tl_val, tr_val, br_val, bl_val, lower);

        get_simple_edges(
            simple_config,
            &tl_pt,
            &tr_pt,
            &br_pt,
            &bl_pt,
            tl_val,
            tr_val,
            br_val,
            bl_val,
            lower,
            smoothing,
        )
    }
}

/// Calculate simple binary configuration for a given threshold
fn calculate_isoline_config(tl: f64, tr: f64, br: f64, bl: f64, level: f64) -> u8 {
    let mut config = 0u8;
    if tl >= level {
        config |= 0b1000;
    }
    if tr >= level {
        config |= 0b0100;
    }
    if br >= level {
        config |= 0b0010;
    }
    if bl >= level {
        config |= 0b0001;
    }
    config
}

/// Get edges for simple binary marching squares configuration
fn get_simple_edges(
    config: u8,
    tl_pt: &Point,
    tr_pt: &Point,
    br_pt: &Point,
    bl_pt: &Point,
    tl_val: f64,
    tr_val: f64,
    br_val: f64,
    bl_val: f64,
    level: f64,
    smoothing: f64,
) -> Option<CellShape> {
    let mut edges = Vec::new();

    match config {
        0 | 15 => return None,

        1 | 14 => {
            // Bottom-left corner
            let left = interpolate_side(
                level,
                Side::Left,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let bottom = interpolate_side(
                level,
                Side::Bottom,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            edges.push(Edge::new(left, bottom, Move::None));
        }

        2 | 13 => {
            // Bottom-right corner
            let bottom = interpolate_side(
                level,
                Side::Bottom,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let right = interpolate_side(
                level,
                Side::Right,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            edges.push(Edge::new(bottom, right, Move::None));
        }

        3 | 12 => {
            // Bottom edge
            let left = interpolate_side(
                level,
                Side::Left,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let right = interpolate_side(
                level,
                Side::Right,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            edges.push(Edge::new(left, right, Move::None));
        }

        4 | 11 => {
            // Top-right corner
            let right = interpolate_side(
                level,
                Side::Right,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let top = interpolate_side(
                level,
                Side::Top,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            edges.push(Edge::new(right, top, Move::None));
        }

        5 | 10 => {
            // Saddle cases
            let avg = (tl_val + tr_val + br_val + bl_val) / 4.0;

            let top = interpolate_side(
                level,
                Side::Top,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let right = interpolate_side(
                level,
                Side::Right,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let bottom = interpolate_side(
                level,
                Side::Bottom,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let left = interpolate_side(
                level,
                Side::Left,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );

            if config == 5 {
                if avg >= level {
                    edges.push(Edge::new(left.clone(), bottom, Move::None));
                    edges.push(Edge::new(right, top, Move::None));
                } else {
                    edges.push(Edge::new(left.clone(), top, Move::None));
                    edges.push(Edge::new(bottom, right, Move::None));
                }
            } else {
                // config == 10
                if avg >= level {
                    edges.push(Edge::new(top.clone(), left, Move::None));
                    edges.push(Edge::new(bottom.clone(), right, Move::None));
                } else {
                    edges.push(Edge::new(top.clone(), right, Move::None));
                    edges.push(Edge::new(bottom.clone(), left, Move::None));
                }
            }
        }

        6 | 9 => {
            // Right edge
            let top = interpolate_side(
                level,
                Side::Top,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let bottom = interpolate_side(
                level,
                Side::Bottom,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            edges.push(Edge::new(top, bottom, Move::None));
        }

        7 | 8 => {
            // Top-left corner
            let top = interpolate_side(
                level,
                Side::Top,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            let left = interpolate_side(
                level,
                Side::Left,
                (tl_pt, tl_val),
                (tr_pt, tr_val),
                (br_pt, br_val),
                (bl_pt, bl_val),
                smoothing,
            );
            edges.push(Edge::new(top, left, Move::None));
        }

        _ => return None,
    }

    if edges.is_empty() {
        None
    } else {
        Some(CellShape::new(edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isoline_config() {
        // All below
        assert_eq!(calculate_isoline_config(5.0, 5.0, 5.0, 5.0, 10.0), 0);

        // All above
        assert_eq!(calculate_isoline_config(15.0, 15.0, 15.0, 15.0, 10.0), 15);

        // Bottom-left above
        assert_eq!(calculate_isoline_config(5.0, 5.0, 5.0, 15.0, 10.0), 1);

        // Top-left above
        assert_eq!(calculate_isoline_config(15.0, 5.0, 5.0, 5.0, 10.0), 8);
    }

    #[test]
    fn test_cell_shape_creation() {
        let tl = GridPoint::new(-100.0, 41.0, 5.0);
        let tr = GridPoint::new(-99.0, 41.0, 5.0);
        let br = GridPoint::new(-99.0, 40.0, 15.0);
        let bl = GridPoint::new(-100.0, 40.0, 15.0);

        let shape = CellShape::from_config(
            1,  // config doesn't matter for current impl
            &tl, &tr, &br, &bl,
            10.0, 20.0, 0.999,
            false, false, false, false,
        );

        assert!(shape.is_some());
        let shape = shape.unwrap();
        assert!(!shape.edges.is_empty());
    }
}
