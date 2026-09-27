# solver_3d

## Classs

- [Constraint3D](Constraint3D.md) — 3D Geometric Constraint between entities.
- [ConstraintSolver3D](ConstraintSolver3D.md) — Variational 3D Assembly & Sketch Constraint Solver.
- [DualQuaternion](DualQuaternion.md) — Unit Dual Quaternion representing a rigid body pose in SE(3).
- [SolverConfig](SolverConfig.md) — Configuration for Levenberg-Marquardt solver.
- [SolverStatus](SolverStatus.md) — Status of the constraint system.

## Functions

- [add_body](add_body.md) — Add a body with initial pose and fixed status.
- [add_body](add_body_1.md) — Add a body with initial pose and fixed status.
- [add_constraint](add_constraint.md) — Add a 3D geometric constraint.
- [add_constraint](add_constraint_1.md) — Add a 3D geometric constraint.
- [compute_residuals](compute_residuals.md) — Evaluate scalar residual vector across all constraints.
- [compute_residuals](compute_residuals_1.md) — Evaluate scalar residual vector across all constraints.
- [count_equations](count_equations.md) — Count total scalar residual equations across all constraints.
- [count_equations](count_equations_1.md) — Count total scalar residual equations across all constraints.
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [from_axis_angle](from_axis_angle.md) — Construct from rotation axis [ax, ay, az] and angle in radians.
- [from_axis_angle](from_axis_angle_1.md) — Construct from rotation axis [ax, ay, az] and angle in radians.
- [from_translation](from_translation.md) — Construct from pure translation [tx, ty, tz].
- [from_translation](from_translation_1.md) — Construct from pure translation [tx, ty, tz].
- [from_translation_rotation](from_translation_rotation.md) — Construct from translation vector [tx, ty, tz] and unit quaternion [qw, qx, qy, qz].
- [from_translation_rotation](from_translation_rotation_1.md) — Construct from translation vector [tx, ty, tz] and unit quaternion [qw, qx, qy, qz].
- [identity](identity.md) — Identity pose (zero translation, zero rotation).
- [identity](identity_1.md) — Identity pose (zero translation, zero rotation).
- [multiply](multiply.md) — Dual quaternion multiplication: self * other.
- [multiply](multiply_1.md) — Dual quaternion multiplication: self * other.
- [new](new.md) — Create a new 3D constraint solver.
- [new](new_1.md) — Create a new 3D constraint solver.
- [px_dummy](px_dummy.md) — [inline(always)]
- [retract](retract.md) — Retract tangent vector xi in se(3) to SE(3) manifold via exponential map.
- [retract](retract_1.md) — Retract tangent vector xi in se(3) to SE(3) manifold via exponential map.
- [solve](solve.md) — Solve the 3D constraint system using Levenberg-Marquardt with adaptive damping.
- [solve](solve_1.md) — Solve the 3D constraint system using Levenberg-Marquardt with adaptive damping.
- [test_distance_and_parallel_axes](test_distance_and_parallel_axes.md) — [test]
- [test_point_coincidence_solve](test_point_coincidence_solve.md) — [test]
- [transform_point](transform_point.md) — Transform a 3D point using this dual quaternion.
- [transform_point](transform_point_1.md) — Transform a 3D point using this dual quaternion.
- [transform_vector](transform_vector.md) — Transform a 3D direction vector (rotation only).
- [transform_vector](transform_vector_1.md) — Transform a 3D direction vector (rotation only).
- [translation](translation.md) — Extract translation vector [tx, ty, tz].
- [translation](translation_1.md) — Extract translation vector [tx, ty, tz].
