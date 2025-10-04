//! Square shape implementation (1 function)

use crate::types::{Edge, Move, Point, Side};

// Case 85 (1111) - Full square
#[allow(clippy::too_many_arguments)]
pub(super) fn square_85(
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
