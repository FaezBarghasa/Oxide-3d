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

/// Exact adaptive 3D orientation predicate using robust floating point arithmetic.
/// Returns positive if pd lies below plane pa-pb-pc, negative if above, zero if coplanar.
#[must_use]
pub fn orient3d(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3]) -> f64 {
    robust::orient3d(
        robust::Coord3D { x: pa[0], y: pa[1], z: pa[2] },
        robust::Coord3D { x: pb[0], y: pb[1], z: pb[2] },
        robust::Coord3D { x: pc[0], y: pc[1], z: pc[2] },
        robust::Coord3D { x: pd[0], y: pd[1], z: pd[2] },
    )
}
