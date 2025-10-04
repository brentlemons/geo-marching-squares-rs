//! Triangle shape implementations (4 functions)
//!
//! Triangle shapes occur when three corners are on one side of a threshold
//! and one corner is on the other side.

use crate::types::{Edge, Move, Point};

// Case 169 | 1 (2221 | 0001) - Bottom-left triangle
pub(super) fn triangle_bl(edges: &mut Vec<Edge>, points: &[Point], is_bottom: bool, is_left: bool) {
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
pub(super) fn triangle_br(edges: &mut Vec<Edge>, points: &[Point], is_right: bool, is_bottom: bool) {
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
pub(super) fn triangle_tr(edges: &mut Vec<Edge>, points: &[Point], is_right: bool, is_top: bool) {
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
pub(super) fn triangle_tl(edges: &mut Vec<Edge>, points: &[Point], is_left: bool, is_top: bool) {
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
