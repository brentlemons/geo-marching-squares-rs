//! Cell shape configurations for 3-level marching squares
//!
//! This is a direct port of the Java implementation from marching-squares-java.
//! Each corner can be in one of 3 states: below lower (0), between thresholds (1), or above upper (2).
//!
//! Configuration encoding: TL(0/64/128) | TR(0/16/32) | BR(0/4/8) | BL(0/1/2)
//! This creates 81 possible configurations mapped to values 0-170.

use crate::interpolation::interpolate_with_method;
use crate::types::{Edge, GridPoint, InterpolationMethod, Move, Point, Side};
use std::collections::HashMap;
use std::fmt;

/// Cell configuration value (0-170 for 3-level encoding)
pub type CellConfig = u8;

/// Represents the edges for a marching squares cell
#[derive(Clone)]
pub struct CellShape {
    /// Edges in this cell, keyed by start point (matches Java HashMap implementation)
    pub edges: HashMap<Point, Edge>,
}

impl fmt::Debug for CellShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CellShape")
            .field("edge_count", &self.edges.len())
            .finish_non_exhaustive()
    }
}

impl CellShape {
    /// Create a cell shape directly from edges (for testing)
    /// Converts Vec<Edge> to HashMap<Point, Edge> keyed by start point
    pub fn new(edges: Vec<Edge>) -> Self {
        let mut edge_map = HashMap::new();
        for edge in edges {
            edge_map.insert(edge.start, edge);
        }
        Self { edges: edge_map }
    }

    /// Create a cell shape directly from a HashMap (for direct construction)
    pub fn new_from_map(edges: HashMap<Point, Edge>) -> Self {
        Self { edges }
    }

    /// Create edges for this cell configuration using full 81-case logic from Java
    pub fn from_config(
        config: CellConfig,
        tl: &GridPoint,
        tr: &GridPoint,
        br: &GridPoint,
        bl: &GridPoint,
        lower: f64,
        upper: f64,
        smoothing: f64,
        interpolation_method: InterpolationMethod,
        is_top_edge: bool,
        is_right_edge: bool,
        is_bottom_edge: bool,
        is_left_edge: bool,
    ) -> Option<Self> {
        // Empty cells (all below or all above)
        if config == 0 || config == 170 {
            return None;
        }

        // Get corner points and values
        let tl_pt = Point::from_lon_lat(tl.lon, tl.lat);
        let tr_pt = Point::from_lon_lat(tr.lon, tr.lat);
        let br_pt = Point::from_lon_lat(br.lon, br.lat);
        let bl_pt = Point::from_lon_lat(bl.lon, bl.lat);

        let tl_val = tl.value as f64;
        let tr_val = tr.value as f64;
        let br_val = br.value as f64;
        let bl_val = bl.value as f64;

        // Helper function to check if an edge is blank (both corners on same side of threshold)
        let is_top_blank = || ((tl_val >= upper) && (tr_val >= upper)) || ((tl_val < lower) && (tr_val < lower));
        let is_right_blank = || ((tr_val >= upper) && (br_val >= upper)) || ((tr_val < lower) && (br_val < lower));
        let is_bottom_blank = || ((bl_val >= upper) && (br_val >= upper)) || ((bl_val < lower) && (br_val < lower));
        let is_left_blank = || ((tl_val >= upper) && (bl_val >= upper)) || ((tl_val < lower) && (bl_val < lower));

        // Helper function to interpolate on a side using the selected method
        let interp = |level: f64, side: Side| -> Point {
            match side {
                Side::Top => interpolate_with_method(interpolation_method, level, tl_val, tr_val, &tl_pt, &tr_pt, smoothing),
                Side::Right => interpolate_with_method(interpolation_method, level, tr_val, br_val, &tr_pt, &br_pt, smoothing),
                Side::Bottom => interpolate_with_method(interpolation_method, level, bl_val, br_val, &bl_pt, &br_pt, smoothing),
                Side::Left => interpolate_with_method(interpolation_method, level, tl_val, bl_val, &tl_pt, &bl_pt, smoothing),
            }
        };

        // Generate the 8 candidate points (matching Java logic exactly)
        // These represent potential edge crossing points in clockwise order starting from top-right
        let mut eight_points: Vec<Option<Point>> = vec![
            // 0: Top edge at TR corner
            if !is_top_blank() {
                Some(if tr_val >= upper { interp(upper, Side::Top) }
                     else if tr_val < lower { interp(lower, Side::Top) }
                     else { tr_pt.clone() })
            } else { None },
            // 1: Right edge at TR corner
            if !is_right_blank() {
                Some(if tr_val >= upper { interp(upper, Side::Right) }
                     else if tr_val < lower { interp(lower, Side::Right) }
                     else { tr_pt.clone() })
            } else { None },
            // 2: Right edge at BR corner
            if !is_right_blank() {
                Some(if br_val >= upper { interp(upper, Side::Right) }
                     else if br_val < lower { interp(lower, Side::Right) }
                     else { br_pt.clone() })
            } else { None },
            // 3: Bottom edge at BR corner
            if !is_bottom_blank() {
                Some(if br_val >= upper { interp(upper, Side::Bottom) }
                     else if br_val < lower { interp(lower, Side::Bottom) }
                     else { br_pt.clone() })
            } else { None },
            // 4: Bottom edge at BL corner
            if !is_bottom_blank() {
                Some(if bl_val >= upper { interp(upper, Side::Bottom) }
                     else if bl_val < lower { interp(lower, Side::Bottom) }
                     else { bl_pt.clone() })
            } else { None },
            // 5: Left edge at BL corner
            if !is_left_blank() {
                Some(if bl_val >= upper { interp(upper, Side::Left) }
                     else if bl_val < lower { interp(lower, Side::Left) }
                     else { bl_pt.clone() })
            } else { None },
            // 6: Left edge at TL corner
            if !is_left_blank() {
                Some(if tl_val >= upper { interp(upper, Side::Left) }
                     else if tl_val < lower { interp(lower, Side::Left) }
                     else { tl_pt.clone() })
            } else { None },
            // 7: Top edge at TL corner
            if !is_top_blank() {
                Some(if tl_val >= upper { interp(upper, Side::Top) }
                     else if tl_val < lower { interp(lower, Side::Top) }
                     else { tl_pt.clone() })
            } else { None },
        ];

        // Filter nulls and deduplicate (matching Java's .distinct().filter())
        let mut points: Vec<Point> = Vec::new();
        for opt_pt in eight_points.iter_mut() {
            if let Some(pt) = opt_pt.take() {
                // Only add if not already present (deduplication)
                if !points.iter().any(|existing| {
                    const EPSILON: f64 = 1e-9;
                    (existing.x - pt.x).abs() < EPSILON && (existing.y - pt.y).abs() < EPSILON
                }) {
                    points.push(pt);
                }
            }
        }

        let mut edges = Vec::new();

        // Route to appropriate shape handler based on config value
        match config {
            // Triangle cases (8 total)
            169 | 1 => triangle_bl(&mut edges, &points, is_bottom_edge, is_left_edge),
            166 | 4 => triangle_br(&mut edges, &points, is_right_edge, is_bottom_edge),
            154 | 16 => triangle_tr(&mut edges, &points, is_right_edge, is_top_edge),
            106 | 64 => triangle_tl(&mut edges, &points, is_left_edge, is_top_edge),

            // Pentagon cases (24 total)
            101 | 69 => pentagon_101(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            149 | 21 => pentagon_149(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            86 | 84 => pentagon_86(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            89 | 81 => pentagon_89(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            96 | 74 => pentagon_96(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            24 | 146 => pentagon_24(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            6 | 164 => pentagon_6(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            129 | 41 => pentagon_129(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            66 | 104 => pentagon_66(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            144 | 26 => pentagon_144(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            36 | 134 => pentagon_36(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            9 | 161 => pentagon_9(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),

            // Rectangle cases (12 total)
            5 | 165 => rectangle_5(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            20 | 150 => rectangle_20(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            80 | 90 => rectangle_80(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            65 | 105 => rectangle_65(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            160 | 10 => rectangle_160(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            130 | 40 => rectangle_130(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),

            // Trapezoid cases (8 total)
            168 | 2 => trapezoid_168(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            162 | 8 => trapezoid_162(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            138 | 32 => trapezoid_138(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            42 | 128 => trapezoid_42(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),

            // Hexagon cases (12 total)
            37 | 133 => hexagon_37(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            148 | 22 => hexagon_148(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            82 | 88 => hexagon_82(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            73 | 97 => hexagon_73(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            145 | 25 => hexagon_145(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),
            70 | 100 => hexagon_70(&mut edges, &points, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge),

            // Saddle cases (14 total) - these are complex with average calculations
            153 => saddle_153(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            102 => saddle_102(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            68 => saddle_68(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            17 => saddle_17(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            136 => saddle_136(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            34 => saddle_34(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            152 => saddle_152(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            18 => saddle_18(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            137 => saddle_137(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            33 => saddle_33(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            98 => saddle_98(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            72 => saddle_72(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            38 => saddle_38(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),
            132 => saddle_132(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),

            // Square case (1 total)
            85 => square_85(&mut edges, &tl_pt, &tr_pt, &br_pt, &bl_pt, tl_val, tr_val, br_val, bl_val, lower, upper, smoothing, is_top_edge, is_right_edge, is_bottom_edge, is_left_edge, &interp),

            _ => return None,
        }

        if edges.is_empty() {
            None
        } else {
            Some(Self::new(edges))
        }
    }
}

// ============================================================================
// Triangle implementations (4 functions)
// ============================================================================

// Case 169 | 1 (2221 | 0001) - Bottom-left triangle
fn triangle_bl(edges: &mut Vec<Edge>, points: &[Point], is_bottom: bool, is_left: bool) {
    // Java reference: points.get(0), points.get(1), points.get(2)
    if points.len() < 3 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];

    if is_bottom {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p0.clone(), Move::Down));
}

// Case 166 | 4 (2212 | 0010) - Bottom-right triangle
fn triangle_br(edges: &mut Vec<Edge>, points: &[Point], is_right: bool, is_bottom: bool) {
    // Java reference: points.get(0), points.get(1), points.get(2)
    if points.len() < 3 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p0.clone(), Move::Right));
}

// Case 154 | 16 (2122 | 0100) - Top-right triangle
fn triangle_tr(edges: &mut Vec<Edge>, points: &[Point], is_right: bool, is_top: bool) {
    // Java reference: points.get(0), points.get(1), points.get(2)
    if points.len() < 3 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p2.clone(), p0.clone(), Move::Right));
    }
}

// Case 106 | 64 (1222 | 1000) - Top-left triangle
fn triangle_tl(edges: &mut Vec<Edge>, points: &[Point], is_left: bool, is_top: bool) {
    // Java reference: points.get(0), points.get(1), points.get(2)
    if points.len() < 3 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p2.clone(), p0.clone(), Move::None));
    }
}

// ============================================================================
// Pentagon implementations (12 functions)
// ============================================================================

// Case 101 | 69 (1211 | 1011)
fn pentagon_101(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    if is_right {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 149 | 21 (2111 | 0111)
fn pentagon_149(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 86 | 84 (1112 | 1110)
fn pentagon_86(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 89 | 81 (1121 | 1101)
fn pentagon_89(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 96 | 74 (1200 | 1022)
fn pentagon_96(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    if is_right {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 24 | 146 (0120 | 2102)
fn pentagon_24(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 6 | 164 (0012 | 2210)
fn pentagon_6(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
}

// Case 129 | 41 (2001 | 0221)
fn pentagon_129(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 66 | 104 (1002 | 1220)
fn pentagon_66(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 144 | 26 (2100 | 0122)
fn pentagon_144(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 36 | 134 (0210 | 2012)
fn pentagon_36(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    if is_right {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 9 | 161 (0021 | 2201)
fn pentagon_9(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
}

// ============================================================================
// Rectangle implementations (6 functions)
// ============================================================================

// Case 5 | 165 (0011 | 2211)
fn rectangle_5(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
}

// Case 20 | 150 (0110 | 2112)
fn rectangle_20(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
    }
}

// Case 80 | 90 (1100 | 1122)
fn rectangle_80(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
    }
}

// Case 65 | 105 (1001 | 1221)
fn rectangle_65(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// Case 160 | 10 (2200 | 0022)
fn rectangle_160(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
}

// Case 130 | 40 (2002 | 0220)
fn rectangle_130(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// ============================================================================
// Trapezoid implementations (4 functions)
// ============================================================================

// Case 168 | 2 (2220 | 0002)
fn trapezoid_168(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_bottom {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p0.clone(), Move::Down));
}

// Case 162 | 8 (2202 | 0020)
fn trapezoid_162(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
}

// Case 138 | 32 (2022 | 0200)
fn trapezoid_138(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    if is_right {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// Case 42 | 128 (0222 | 2000)
fn trapezoid_42(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
    if is_left {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// ============================================================================
// Hexagon implementations (6 functions)
// ============================================================================

// Case 37 | 133 (0211 | 2011)
fn hexagon_37(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    if is_right { edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down)); }
    if is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left)); }
    if is_left { edges.push(Edge::new(p3.clone(), p4.clone(), Move::None)); }
    edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up));
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::None)); }
}

// Case 148 | 22 (2110 | 0112)
fn hexagon_148(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down)); }
    if is_bottom { edges.push(Edge::new(p1.clone(), p2.clone(), Move::None)); }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    if is_left { edges.push(Edge::new(p3.clone(), p4.clone(), Move::None)); }
    edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up));
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::Right)); }
}

// Case 82 | 88 (1102 | 1120)
fn hexagon_82(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::None)); }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    if is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::None)); }
    edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
    if is_left { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::Right)); }
}

// Case 73 | 97 (1021 | 1201)
fn hexagon_73(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    if is_right { edges.push(Edge::new(p1.clone(), p2.clone(), Move::None)); }
    edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
    if is_bottom { edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left)); }
    if is_left { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::None)); }
}

// Case 145 | 25 (2101 | 0121)
fn hexagon_145(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::None)); }
    edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    if is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left)); }
    if is_left { edges.push(Edge::new(p3.clone(), p4.clone(), Move::None)); }
    edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up));
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::Right)); }
}

// Case 70 | 100 (1012 | 1210)
fn hexagon_70(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    if is_right { edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down)); }
    if is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::None)); }
    edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
    if is_left { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::None)); }
}

// ============================================================================
// Saddle implementations (14 functions) - Complex cases with average calculations
// ============================================================================

// Case 153 (2121)
#[allow(clippy::too_many_arguments)]
fn saddle_153(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, tr_pt: &Point, _br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average >= upper {
        let p0 = interp(upper, Side::Right);
        let p1 = interp(upper, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(upper, Side::Left);
        let p4 = interp(upper, Side::Bottom);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p4.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(upper, Side::Right);
        let p1 = interp(upper, Side::Bottom);
        let p3 = interp(upper, Side::Left);
        let p4 = interp(upper, Side::Top);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p1.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p3.clone(), Move::None));
        }
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p4.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 102 (1212)
#[allow(clippy::too_many_arguments)]
fn saddle_102(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, _tr_pt: &Point, br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average >= upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Left);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(upper, Side::Bottom);
        let p4 = interp(upper, Side::Right);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p4.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Right);
        let p3 = interp(upper, Side::Bottom);
        let p4 = interp(upper, Side::Left);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p4.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 68 (1010)
#[allow(clippy::too_many_arguments)]
fn saddle_68(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, _tr_pt: &Point, br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Left);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(lower, Side::Bottom);
        let p4 = interp(lower, Side::Right);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p4.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Right);
        let p3 = interp(lower, Side::Bottom);
        let p4 = interp(lower, Side::Left);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p4.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 17 (0101)
#[allow(clippy::too_many_arguments)]
fn saddle_17(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, tr_pt: &Point, _br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower {
        let p0 = interp(lower, Side::Right);
        let p1 = interp(lower, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(lower, Side::Left);
        let p4 = interp(lower, Side::Bottom);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p4.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(lower, Side::Right);
        let p1 = interp(lower, Side::Bottom);
        let p3 = interp(lower, Side::Left);
        let p4 = interp(lower, Side::Top);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p1.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p3.clone(), Move::None));
        }
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p4.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 136 (2020) - 8 point case
#[allow(clippy::too_many_arguments)]
fn saddle_136(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, _tr_pt: &Point, _br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Left);
        let p2 = interp(upper, Side::Left);
        let p3 = interp(upper, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = interp(upper, Side::Right);
        let p5 = interp(upper, Side::Bottom);
        let p6 = interp(lower, Side::Bottom);
        let p7 = interp(lower, Side::Right);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p7, p4, Move::None));
        }
    } else if average >= upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Right);
        let p2 = interp(upper, Side::Right);
        let p3 = interp(upper, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = interp(lower, Side::Bottom);
        let p5 = interp(lower, Side::Left);
        let p6 = interp(upper, Side::Left);
        let p7 = interp(upper, Side::Bottom);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p7, p4, Move::None));
        }
    } else {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Right);
        let p2 = interp(upper, Side::Right);
        let p3 = interp(upper, Side::Bottom);
        let p4 = interp(lower, Side::Bottom);
        let p5 = interp(lower, Side::Left);
        let p6 = interp(upper, Side::Left);
        let p7 = interp(upper, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p7, p0, Move::None));
        }
    }
}

// Case 34 (0202) - 8 point case
#[allow(clippy::too_many_arguments)]
fn saddle_34(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, _tr_pt: &Point, _br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average >= upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Left);
        let p2 = interp(lower, Side::Left);
        let p3 = interp(lower, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = interp(lower, Side::Right);
        let p5 = interp(lower, Side::Bottom);
        let p6 = interp(upper, Side::Bottom);
        let p7 = interp(upper, Side::Right);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p7, p4, Move::None));
        }
    } else if average < lower {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Right);
        let p2 = interp(lower, Side::Right);
        let p3 = interp(lower, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = interp(upper, Side::Bottom);
        let p5 = interp(upper, Side::Left);
        let p6 = interp(lower, Side::Left);
        let p7 = interp(lower, Side::Bottom);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p7, p4, Move::None));
        }
    } else {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Right);
        let p2 = interp(lower, Side::Right);
        let p3 = interp(lower, Side::Bottom);
        let p4 = interp(upper, Side::Bottom);
        let p5 = interp(upper, Side::Left);
        let p6 = interp(lower, Side::Left);
        let p7 = interp(lower, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p7, p0, Move::None));
        }
    }
}

// Case 152 (2120) - 7 point case
#[allow(clippy::too_many_arguments)]
fn saddle_152(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, tr_pt: &Point, _br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = interp(upper, Side::Right);
        let p1 = interp(upper, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(lower, Side::Bottom);
        let p4 = interp(lower, Side::Left);
        let p5 = interp(upper, Side::Left);
        let p6 = interp(upper, Side::Bottom);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p6, p3, Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(upper, Side::Right);
        let p1 = interp(upper, Side::Bottom);
        let p2 = interp(lower, Side::Bottom);
        let p3 = interp(lower, Side::Left);
        let p4 = interp(upper, Side::Left);
        let p5 = interp(upper, Side::Top);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p5.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 18 (0102) - 7 point case
#[allow(clippy::too_many_arguments)]
fn saddle_18(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, tr_pt: &Point, _br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = interp(lower, Side::Right);
        let p1 = interp(lower, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(upper, Side::Bottom);
        let p4 = interp(upper, Side::Left);
        let p5 = interp(lower, Side::Left);
        let p6 = interp(lower, Side::Bottom);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p6, p3, Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(lower, Side::Right);
        let p1 = interp(lower, Side::Bottom);
        let p2 = interp(upper, Side::Bottom);
        let p3 = interp(upper, Side::Left);
        let p4 = interp(lower, Side::Left);
        let p5 = interp(lower, Side::Top);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p5.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 137 (2021) - 7 point case
#[allow(clippy::too_many_arguments)]
fn saddle_137(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, _tr_pt: &Point, _br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Right);
        let p2 = interp(upper, Side::Right);
        let p3 = interp(upper, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = interp(upper, Side::Left);
        let p5 = interp(upper, Side::Bottom);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p5.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p4.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Right);
        let p2 = interp(upper, Side::Right);
        let p3 = interp(upper, Side::Bottom);
        let p5 = interp(upper, Side::Left);
        let p6 = interp(upper, Side::Top);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p3.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p6.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 33 (0201) - 7 point case
#[allow(clippy::too_many_arguments)]
fn saddle_33(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, _tr_pt: &Point, _br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Right);
        let p2 = interp(lower, Side::Right);
        let p3 = interp(lower, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = interp(lower, Side::Left);
        let p5 = interp(lower, Side::Bottom);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p5.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p4.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Right);
        let p2 = interp(lower, Side::Right);
        let p3 = interp(lower, Side::Bottom);
        let p5 = interp(lower, Side::Left);
        let p6 = interp(lower, Side::Top);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p3.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p6.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 98 (1202) - 7 point case
#[allow(clippy::too_many_arguments)]
fn saddle_98(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, _tr_pt: &Point, _br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Left);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(lower, Side::Right);
        let p4 = interp(lower, Side::Bottom);
        let p5 = interp(upper, Side::Bottom);
        let p6 = interp(upper, Side::Right);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p6, p3, Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Right);
        let p2 = interp(lower, Side::Right);
        let p3 = interp(lower, Side::Bottom);
        let p4 = interp(upper, Side::Bottom);
        let p5 = interp(upper, Side::Left);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p5.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 72 (1020) - 7 point case
#[allow(clippy::too_many_arguments)]
fn saddle_72(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, _tr_pt: &Point, _br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Left);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(upper, Side::Right);
        let p4 = interp(upper, Side::Bottom);
        let p5 = interp(lower, Side::Bottom);
        let p6 = interp(lower, Side::Right);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p6, p3, Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Right);
        let p2 = interp(upper, Side::Right);
        let p3 = interp(upper, Side::Bottom);
        let p4 = interp(lower, Side::Bottom);
        let p5 = interp(lower, Side::Left);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p5.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 38 (0212) - 7 point case
#[allow(clippy::too_many_arguments)]
fn saddle_38(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, _tr_pt: &Point, br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Left);
        let p2 = interp(lower, Side::Left);
        let p3 = interp(lower, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = interp(upper, Side::Bottom);
        let p5 = interp(upper, Side::Right);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p5.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p4.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Right);
        let p3 = interp(upper, Side::Bottom);
        let p4 = interp(upper, Side::Left);
        let p5 = interp(lower, Side::Left);
        let p6 = interp(lower, Side::Top);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p6.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 132 (2010) - 7 point case
#[allow(clippy::too_many_arguments)]
fn saddle_132(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, _tr_pt: &Point, br_pt: &Point, _bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Left);
        let p2 = interp(upper, Side::Left);
        let p3 = interp(upper, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = interp(lower, Side::Bottom);
        let p5 = interp(lower, Side::Right);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p5.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p4.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Right);
        let p3 = interp(lower, Side::Bottom);
        let p4 = interp(lower, Side::Left);
        let p5 = interp(upper, Side::Left);
        let p6 = interp(upper, Side::Top);

        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p6.clone(), p0.clone(), Move::None));
        }
    }
}

// ============================================================================
// Square implementation (1 function)
// ============================================================================

// Case 85 (1111) - Full square
#[allow(clippy::too_many_arguments)]
fn square_85(
    edges: &mut Vec<Edge>,
    _tl_pt: &Point, _tr_pt: &Point, _br_pt: &Point, _bl_pt: &Point,
    _tl_val: f64, _tr_val: f64, _br_val: f64, _bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    interp: &impl Fn(f64, Side) -> Point,
) {
    let p0 = interp(upper, Side::Right);
    let p1 = interp(upper, Side::Bottom);
    let p2 = interp(upper, Side::Left);
    let p3 = interp(upper, Side::Top);

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p3, p0, Move::Right));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_cells() {
        let tl = GridPoint::new(0.0, 1.0, 0.0);
        let tr = GridPoint::new(1.0, 1.0, 0.0);
        let br = GridPoint::new(1.0, 0.0, 0.0);
        let bl = GridPoint::new(0.0, 0.0, 0.0);

        // All below lower
        let result = CellShape::from_config(0, &tl, &tr, &br, &bl, 5.0, 10.0, 0.999, InterpolationMethod::Cosine, false, false, false, false);
        assert!(result.is_none());

        // All above upper
        let result = CellShape::from_config(170, &tl, &tr, &br, &bl, 5.0, 10.0, 0.999, InterpolationMethod::Cosine, false, false, false, false);
        assert!(result.is_none());
    }

    #[test]
    fn test_triangle_config() {
        let tl = GridPoint::new(0.0, 1.0, 12.0);
        let tr = GridPoint::new(1.0, 1.0, 12.0);
        let br = GridPoint::new(1.0, 0.0, 12.0);
        let bl = GridPoint::new(0.0, 0.0, 4.0);

        // Config 169 (2221) - all above upper except BL between
        let result = CellShape::from_config(169, &tl, &tr, &br, &bl, 5.0, 10.0, 0.999, InterpolationMethod::Cosine, false, false, false, false);
        assert!(result.is_some());
        let shape = result.unwrap();
        assert!(shape.edges.len() > 0);
    }
}
