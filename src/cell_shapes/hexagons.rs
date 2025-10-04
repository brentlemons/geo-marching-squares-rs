//! Hexagon shape implementations (6 functions)

use crate::types::{Edge, Move, Point};

// Case 37 | 133 (0211 | 2011)
pub(super) fn hexagon_37(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if !is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right)); } else { edges.push(Edge::new(p0.clone(), p1.clone(), Move::None)); }
    if is_right { edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down)); }
    if is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left)); }
    if is_left { edges.push(Edge::new(p3.clone(), p4.clone(), Move::None)); }
    if !is_top { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); } else { edges.push(Edge::new(p4.clone(), p5.clone(), Move::None)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::None)); }
}

// Case 148 | 22 (2110 | 0112)
pub(super) fn hexagon_148(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down)); }
    if is_bottom { edges.push(Edge::new(p1.clone(), p2.clone(), Move::None)); }
    if !is_left { edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left)); } else { edges.push(Edge::new(p2.clone(), p3.clone(), Move::None)); }
    if is_left { edges.push(Edge::new(p3.clone(), p4.clone(), Move::None)); }
    if !is_top { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); } else { edges.push(Edge::new(p4.clone(), p5.clone(), Move::None)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::Right)); }
}

// Case 82 | 88 (1102 | 1120)
pub(super) fn hexagon_82(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::None)); }
    if !is_bottom { edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down)); } else { edges.push(Edge::new(p1.clone(), p2.clone(), Move::None)); }
    if is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::None)); }
    if !is_left { edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left)); } else { edges.push(Edge::new(p3.clone(), p4.clone(), Move::None)); }
    if is_left { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::Right)); }
}

// Case 73 | 97 (1021 | 1201)
pub(super) fn hexagon_73(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if !is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right)); } else { edges.push(Edge::new(p0.clone(), p1.clone(), Move::None)); }
    if is_right { edges.push(Edge::new(p1.clone(), p2.clone(), Move::None)); }
    if !is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down)); } else { edges.push(Edge::new(p2.clone(), p3.clone(), Move::None)); }
    if is_bottom { edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left)); }
    if is_left { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::None)); }
}

// Case 145 | 25 (2101 | 0121)
pub(super) fn hexagon_145(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::None)); }
    if !is_bottom { edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down)); } else { edges.push(Edge::new(p1.clone(), p2.clone(), Move::None)); }
    if is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::Left)); }
    if is_left { edges.push(Edge::new(p3.clone(), p4.clone(), Move::None)); }
    if !is_top { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); } else { edges.push(Edge::new(p4.clone(), p5.clone(), Move::None)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::Right)); }
}

// Case 70 | 100 (1012 | 1210)
pub(super) fn hexagon_70(edges: &mut Vec<Edge>, points: &[Point], is_top: bool, is_right: bool, is_bottom: bool, is_left: bool) {
    if points.len() < 6 { return; }
    let (p0, p1, p2, p3, p4, p5) = (&points[0], &points[1], &points[2], &points[3], &points[4], &points[5]);
    if !is_right { edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right)); } else { edges.push(Edge::new(p0.clone(), p1.clone(), Move::None)); }
    if is_right { edges.push(Edge::new(p1.clone(), p2.clone(), Move::Down)); }
    if is_bottom { edges.push(Edge::new(p2.clone(), p3.clone(), Move::None)); }
    if !is_left { edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left)); } else { edges.push(Edge::new(p3.clone(), p4.clone(), Move::None)); }
    if is_left { edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up)); }
    if is_top { edges.push(Edge::new(p5.clone(), p0.clone(), Move::None)); }
}
