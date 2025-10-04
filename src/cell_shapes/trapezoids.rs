//! Trapezoid shape implementations (4 functions)

use crate::types::{Edge, Move, Point};

// Case 168 | 2 (2220 | 0002)
pub(super) fn trapezoid_168(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_bottom {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if !is_left {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    } else {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if !is_bottom {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::Down));
    } else {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// Case 162 | 8 (2202 | 0020)
pub(super) fn trapezoid_162(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if !is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    } else {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if !is_right {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// Case 138 | 32 (2022 | 0200)
pub(super) fn trapezoid_138(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if !is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if is_right {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    if !is_top {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    } else {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// Case 42 | 128 (0222 | 2000)
pub(super) fn trapezoid_42(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if !is_left {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if is_left {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    if !is_top {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    } else {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}
