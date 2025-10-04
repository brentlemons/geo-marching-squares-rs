//! Rectangle shape implementations (6 functions)

use crate::types::{Edge, Move, Point};

// Case 5 | 165 (0011 | 2211)
pub(super) fn rectangle_5(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
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
    if !is_right {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// Case 20 | 150 (0110 | 2112)
pub(super) fn rectangle_20(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
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
    // Only move UP if not at top boundary
    if !is_top {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    } else {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
    }
}

// Case 80 | 90 (1100 | 1122)
pub(super) fn rectangle_80(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    // Only move LEFT if not at left boundary
    if !is_left {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    } else {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
    }
}

// Case 65 | 105 (1001 | 1221)
pub(super) fn rectangle_65(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if !is_bottom {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
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
pub(super) fn rectangle_160(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if is_right {
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
    if !is_right {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
    }
}

// Case 130 | 40 (2002 | 0220)
pub(super) fn rectangle_130(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 4 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];

    if !is_bottom {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if is_bottom {
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
