//! Saddle shape implementations (14 functions)

use crate::types::{Edge, Move, Point, Side};

// Case 153 (2121)
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_153(
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
        if !is_top {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(upper, Side::Left);
        let p4 = interp(upper, Side::Bottom);
        if !is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
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

        if !is_bottom {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p1.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p3.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
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
pub(super) fn saddle_102(
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
        if !is_left {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(upper, Side::Bottom);
        let p4 = interp(upper, Side::Right);
        if !is_right {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
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

        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
        if !is_left {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
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
pub(super) fn saddle_68(
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
        if !is_left {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(lower, Side::Bottom);
        let p4 = interp(lower, Side::Right);
        if !is_right {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
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

        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
        if !is_left {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
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
pub(super) fn saddle_17(
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
        if !is_top {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = interp(lower, Side::Left);
        let p4 = interp(lower, Side::Bottom);
        if !is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
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

        if !is_bottom {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p1.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p3.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
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
pub(super) fn saddle_136(
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

        let p4 = interp(upper, Side::Right);
        let p5 = interp(upper, Side::Bottom);
        let p6 = interp(lower, Side::Bottom);
        let p7 = interp(lower, Side::Right);
        if !is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if !is_right {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p7, p4, Move::None));
        }
    } else if average >= upper {
        let p0 = interp(lower, Side::Top);
        let p1 = interp(lower, Side::Right);
        let p2 = interp(upper, Side::Right);
        let p3 = interp(upper, Side::Top);
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

        let p4 = interp(lower, Side::Bottom);
        let p5 = interp(lower, Side::Left);
        let p6 = interp(upper, Side::Left);
        let p7 = interp(upper, Side::Bottom);
        if !is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::None));
        }
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
        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if !is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::None));
        }
        if is_top {
            edges.push(Edge::new(p7, p0, Move::None));
        }
    }
}

// Case 34 (0202) - 8 point case
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_34(
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

        let p4 = interp(lower, Side::Right);
        let p5 = interp(lower, Side::Bottom);
        let p6 = interp(upper, Side::Bottom);
        let p7 = interp(upper, Side::Right);
        if !is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if !is_right {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p7, p4, Move::None));
        }
    } else if average < lower {
        let p0 = interp(upper, Side::Top);
        let p1 = interp(upper, Side::Right);
        let p2 = interp(lower, Side::Right);
        let p3 = interp(lower, Side::Top);
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

        let p4 = interp(upper, Side::Bottom);
        let p5 = interp(upper, Side::Left);
        let p6 = interp(lower, Side::Left);
        let p7 = interp(lower, Side::Bottom);
        if !is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::None));
        }
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
        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if !is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p6.clone(), p7.clone(), Move::None));
        }
        if is_top {
            edges.push(Edge::new(p7, p0, Move::None));
        }
    }
}

// Case 152 (2120) - 7 point case
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_152(
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
        if !is_top {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
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
        if !is_left {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
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
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
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
pub(super) fn saddle_18(
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
        if !is_top {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
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
        if !is_left {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
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
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
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
pub(super) fn saddle_137(
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

        let p4 = interp(upper, Side::Left);
        let p5 = interp(upper, Side::Bottom);
        if !is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
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

        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p3.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p5.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if is_top {
            edges.push(Edge::new(p6.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 33 (0201) - 7 point case
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_33(
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

        let p4 = interp(lower, Side::Left);
        let p5 = interp(lower, Side::Bottom);
        if !is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
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

        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p3.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p5.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if is_top {
            edges.push(Edge::new(p6.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 98 (1202) - 7 point case
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_98(
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
        if !is_left {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
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
        if !is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if !is_right {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
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

        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if !is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
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
pub(super) fn saddle_72(
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
        if !is_left {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
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
        if !is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if !is_right {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
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

        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        if !is_bottom {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::Down));
        } else {
            edges.push(Edge::new(p2.clone(), p3.clone(), Move::None));
        }
        if is_bottom {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if !is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
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
pub(super) fn saddle_38(
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

        let p4 = interp(upper, Side::Bottom);
        let p5 = interp(upper, Side::Right);
        if !is_right {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
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

        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
        if !is_left {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if is_top {
            edges.push(Edge::new(p6.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 132 (2010) - 7 point case
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_132(
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

        let p4 = interp(lower, Side::Bottom);
        let p5 = interp(lower, Side::Right);
        if !is_right {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
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

        if !is_right {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        } else {
            edges.push(Edge::new(p0.clone(), p1.clone(), Move::None));
        }
        if is_right {
            edges.push(Edge::new(p1.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
        if !is_left {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        } else {
            edges.push(Edge::new(p3.clone(), p4.clone(), Move::None));
        }
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        if !is_top {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::Up));
        } else {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        if is_top {
            edges.push(Edge::new(p6.clone(), p0.clone(), Move::None));
        }
    }
}
