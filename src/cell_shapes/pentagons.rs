//! Pentagon shape implementations (12 functions)

use crate::types::{Edge, Move, Point};

// Case 101 | 69 (1211 | 1011)
pub(super) fn pentagon_101(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if !is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
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
pub(super) fn pentagon_149(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
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
    if !is_top {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    } else {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 86 | 84 (1112 | 1110)
pub(super) fn pentagon_86(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
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
    if !is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    } else {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 89 | 81 (1121 | 1101)
pub(super) fn pentagon_89(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if !is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    } else {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
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
pub(super) fn pentagon_96(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if !is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if is_right {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    if !is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    } else {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 24 | 146 (0120 | 2102)
pub(super) fn pentagon_24(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

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
    if !is_top {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    } else {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 6 | 164 (0012 | 2210)
pub(super) fn pentagon_6(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
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
    if !is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    } else {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    if !is_right {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 129 | 41 (2001 | 0221)
pub(super) fn pentagon_129(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if !is_bottom {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if !is_top {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    } else {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 66 | 104 (1002 | 1220)
pub(super) fn pentagon_66(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if !is_bottom {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    if !is_left {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    } else {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 144 | 26 (2100 | 0122)
pub(super) fn pentagon_144(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

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
    if !is_top {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    } else {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    }
}

// Case 36 | 134 (0210 | 2012)
pub(super) fn pentagon_36(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if !is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if is_right {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    }
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
    }
    if !is_top {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
    } else {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    if is_top {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}

// Case 9 | 161 (0021 | 2201)
pub(super) fn pentagon_9(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 5 { return; }
    let p0 = &points[0];
    let p1 = &points[1];
    let p2 = &points[2];
    let p3 = &points[3];
    let p4 = &points[4];

    if is_right {
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
    }
    if !is_bottom {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down));
    } else {
        edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
    }
    if is_bottom {
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left));
    }
    if is_left {
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
    }
    if !is_right {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::Right));
    } else {
        edges.push(Edge::new(p4.clone(), p0.clone(), Move::None));
    }
}
