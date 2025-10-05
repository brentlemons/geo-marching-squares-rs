//! Square shape implementation (1 function)

use crate::types::{Edge, Move, Point, Side};

// Case 85 (1111) - Full square
#[allow(clippy::too_many_arguments)]
pub(super) fn square_85(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
    let p1 = get_edge_point(br_pt, br_val, Side::Bottom);
    let p2 = get_edge_point(bl_pt, bl_val, Side::Left);
    let p3 = get_edge_point(tl_pt, tl_val, Side::Top);

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
