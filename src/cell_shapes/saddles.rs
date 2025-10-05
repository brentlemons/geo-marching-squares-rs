//! Saddle shape implementations (14 functions)

use crate::types::{Edge, Move, Point, Side};

// Case 153 (2121)
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_153(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average >= upper {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Bottom);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p4.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p1 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p3 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p4 = get_edge_point(tl_pt, tl_val, Side::Top);

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
pub(super) fn saddle_102(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average >= upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tl_pt, tl_val, Side::Left);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(br_pt, br_val, Side::Right);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p4.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);

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
pub(super) fn saddle_68(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tl_pt, tl_val, Side::Left);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(br_pt, br_val, Side::Right);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p4.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);

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
pub(super) fn saddle_17(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Bottom);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p4.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p1 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p3 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p4 = get_edge_point(tl_pt, tl_val, Side::Top);

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
pub(super) fn saddle_136(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tl_pt, tl_val, Side::Left);
        let p2 = get_edge_point(tl_pt, tl_val, Side::Left);
        let p3 = get_edge_point(tl_pt, tl_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = get_edge_point(br_pt, br_val, Side::Right);
        let p5 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p6 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p7 = get_edge_point(br_pt, br_val, Side::Right);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p7.clone(), p4.clone(), Move::None));
        }
    } else if average >= upper {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(tr_pt, tr_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = get_edge_point(bl_pt, bl_val, Side::Bottom);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p7 = get_edge_point(bl_pt, bl_val, Side::Bottom);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p7.clone(), p4.clone(), Move::None));
        }
    } else {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p7 = get_edge_point(tl_pt, tl_val, Side::Top);
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
            edges.push(Edge::new(p7.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 34 (0202) - 8 point case
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_34(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average >= upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tl_pt, tl_val, Side::Left);
        let p2 = get_edge_point(tl_pt, tl_val, Side::Left);
        let p3 = get_edge_point(tl_pt, tl_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = get_edge_point(br_pt, br_val, Side::Right);
        let p5 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p6 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p7 = get_edge_point(br_pt, br_val, Side::Right);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p7.clone(), p4.clone(), Move::None));
        }
    } else if average < lower {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(tr_pt, tr_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = get_edge_point(bl_pt, bl_val, Side::Bottom);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p7 = get_edge_point(bl_pt, bl_val, Side::Bottom);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p5.clone(), p6.clone(), Move::None));
        }
        edges.push(Edge::new(p6.clone(), p7.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p7.clone(), p4.clone(), Move::None));
        }
    } else {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p7 = get_edge_point(tl_pt, tl_val, Side::Top);
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
            edges.push(Edge::new(p7.clone(), p0.clone(), Move::None));
        }
    }
}

// Case 152 (2120) - 7 point case
#[allow(clippy::too_many_arguments)]
pub(super) fn saddle_152(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(br_pt, br_val, Side::Bottom);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p6.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p1 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p2 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p3 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p5 = get_edge_point(tl_pt, tl_val, Side::Top);
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
pub(super) fn saddle_18(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p1.clone(), tr_pt.clone(), Move::Right));
        }
        if is_right {
            edges.push(Edge::new(tr_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(br_pt, br_val, Side::Bottom);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p6.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p1 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p2 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p3 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p5 = get_edge_point(tl_pt, tl_val, Side::Top);
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
pub(super) fn saddle_137(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(tl_pt, tl_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Bottom);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p5.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p4.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(tl_pt, tl_val, Side::Top);
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
pub(super) fn saddle_33(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(tl_pt, tl_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Bottom);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p5.clone(), bl_pt.clone(), Move::Left));
        }
        if is_left {
            edges.push(Edge::new(bl_pt.clone(), p4.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(tl_pt, tl_val, Side::Top);
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
pub(super) fn saddle_98(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tl_pt, tl_val, Side::Left);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p4 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p6 = get_edge_point(tr_pt, tr_val, Side::Right);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p6.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
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
pub(super) fn saddle_72(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tl_pt, tl_val, Side::Left);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), tl_pt.clone(), Move::Up));
        }
        if is_top {
            edges.push(Edge::new(tl_pt.clone(), p0.clone(), Move::None));
        }

        let p3 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p4 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p6 = get_edge_point(tr_pt, tr_val, Side::Right);
        edges.push(Edge::new(p3.clone(), p4.clone(), Move::Down));
        if is_bottom {
            edges.push(Edge::new(p4.clone(), p5.clone(), Move::None));
        }
        edges.push(Edge::new(p5.clone(), p6.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p6.clone(), p3.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p2 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
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
pub(super) fn saddle_38(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tl_pt, tl_val, Side::Left);
        let p2 = get_edge_point(tl_pt, tl_val, Side::Left);
        let p3 = get_edge_point(tl_pt, tl_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(br_pt, br_val, Side::Right);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p5.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p4.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(tl_pt, tl_val, Side::Top);
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
pub(super) fn saddle_132(
    edges: &mut Vec<Edge>,
    tl_pt: &Point, tr_pt: &Point, br_pt: &Point, bl_pt: &Point,
    tl_val: f64, tr_val: f64, br_val: f64, bl_val: f64,
    lower: f64, upper: f64, _smoothing: f64,
    is_top: bool, is_right: bool, is_bottom: bool, is_left: bool,
    _interp: &impl Fn(f64, Side) -> Point,
    get_edge_point: &impl Fn(&Point, f64, Side) -> Point,
) {
    let average = (tl_val + tr_val + br_val + bl_val) / 4.0;

    if average < lower || average >= upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tl_pt, tl_val, Side::Left);
        let p2 = get_edge_point(tl_pt, tl_val, Side::Left);
        let p3 = get_edge_point(tl_pt, tl_val, Side::Top);
        edges.push(Edge::new(p0.clone(), p1.clone(), Move::Left));
        if is_left {
            edges.push(Edge::new(p1.clone(), p2.clone(), Move::None));
        }
        edges.push(Edge::new(p2.clone(), p3.clone(), Move::Up));
        if is_top {
            edges.push(Edge::new(p3.clone(), p0.clone(), Move::None));
        }

        let p4 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p5 = get_edge_point(br_pt, br_val, Side::Right);
        edges.push(Edge::new(p4.clone(), p5.clone(), Move::Right));
        if is_right {
            edges.push(Edge::new(p5.clone(), br_pt.clone(), Move::Down));
        }
        if is_bottom {
            edges.push(Edge::new(br_pt.clone(), p4.clone(), Move::None));
        }
    } else if average >= lower && average < upper {
        let p0 = get_edge_point(tl_pt, tl_val, Side::Top);
        let p1 = get_edge_point(tr_pt, tr_val, Side::Right);
        let p3 = get_edge_point(br_pt, br_val, Side::Bottom);
        let p4 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p5 = get_edge_point(bl_pt, bl_val, Side::Left);
        let p6 = get_edge_point(tl_pt, tl_val, Side::Top);
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
