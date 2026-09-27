use crate::tolerance::ToleranceContext;

/// Exact adaptive 2D orientation predicate using robust floating point arithmetic.
/// Returns positive if a, b, c are counter-clockwise, negative if clockwise, zero if collinear.
#[must_use]
pub fn orient2d(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2]) -> f64 {
    robust::orient2d(
        robust::Coord { x: pa[0], y: pa[1] },
        robust::Coord { x: pb[0], y: pb[1] },
        robust::Coord { x: pc[0], y: pc[1] },
    )
}

/// Tolerance-aware 2D orientation predicate.
/// Returns 1 if CCW, -1 if CW, 0 if collinear within tolerance.
#[must_use]
pub fn orient2d_tol(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2], tol: &ToleranceContext) -> i8 {
    let val = orient2d(pa, pb, pc);
    if val > tol.linear_epsilon {
        1
    } else if val < -tol.linear_epsilon {
        -1
    } else {
        0
    }
}

/// Exact adaptive 3D orientation predicate using robust floating point arithmetic.
/// Returns positive if pd lies below plane pa-pb-pc, negative if above, zero if coplanar.
#[must_use]
pub fn orient3d(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3]) -> f64 {
    robust::orient3d(
        robust::Coord3D {
            x: pa[0],
            y: pa[1],
            z: pa[2],
        },
        robust::Coord3D {
            x: pb[0],
            y: pb[1],
            z: pb[2],
        },
        robust::Coord3D {
            x: pc[0],
            y: pc[1],
            z: pc[2],
        },
        robust::Coord3D {
            x: pd[0],
            y: pd[1],
            z: pd[2],
        },
    )
}

/// Tolerance-aware 3D orientation predicate.
/// Returns 1 if below, -1 if above, 0 if coplanar within tolerance.
#[must_use]
pub fn orient3d_tol(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3], tol: &ToleranceContext) -> i8 {
    let val = orient3d(pa, pb, pc, pd);
    if val > tol.linear_epsilon {
        1
    } else if val < -tol.linear_epsilon {
        -1
    } else {
        0
    }
}

/// Exact adaptive 2D incircle predicate.
/// Returns positive if pd is inside circle through pa, pb, pc; negative if outside; zero if cocircular.
#[must_use]
pub fn incircle(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2], pd: [f64; 2]) -> f64 {
    robust::incircle(
        robust::Coord { x: pa[0], y: pa[1] },
        robust::Coord { x: pb[0], y: pb[1] },
        robust::Coord { x: pc[0], y: pc[1] },
        robust::Coord { x: pd[0], y: pd[1] },
    )
}

/// Tolerance-aware 2D incircle predicate.
#[must_use]
pub fn incircle_tol(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2], pd: [f64; 2], tol: &ToleranceContext) -> i8 {
    let val = incircle(pa, pb, pc, pd);
    if val > tol.linear_epsilon {
        1
    } else if val < -tol.linear_epsilon {
        -1
    } else {
        0
    }
}

/// Exact adaptive 3D insphere predicate.
/// Returns positive if pe is inside sphere through pa, pb, pc, pd; negative if outside; zero if cospherical.
#[must_use]
pub fn insphere(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3], pe: [f64; 3]) -> f64 {
    robust::insphere(
        robust::Coord3D {
            x: pa[0],
            y: pa[1],
            z: pa[2],
        },
        robust::Coord3D {
            x: pb[0],
            y: pb[1],
            z: pb[2],
        },
        robust::Coord3D {
            x: pc[0],
            y: pc[1],
            z: pc[2],
        },
        robust::Coord3D {
            x: pd[0],
            y: pd[1],
            z: pd[2],
        },
        robust::Coord3D {
            x: pe[0],
            y: pe[1],
            z: pe[2],
        },
    )
}

/// Tolerance-aware 3D insphere predicate.
#[must_use]
pub fn insphere_tol(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3], pe: [f64; 3], tol: &ToleranceContext) -> i8 {
    let val = insphere(pa, pb, pc, pd, pe);
    if val > tol.linear_epsilon {
        1
    } else if val < -tol.linear_epsilon {
        -1
    } else {
        0
    }
}
