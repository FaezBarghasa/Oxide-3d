---
okf_version: "0.2"
type: Module
title: solver_3d
description: "3D Variational Geometric & Assembly Constraint Solver."
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d
language: rust
---

# solver_3d

3D Variational Geometric & Assembly Constraint Solver.

## Docstring

3D Variational Geometric & Assembly Constraint Solver.

Uses unit Dual Quaternions on SE(3) Riemannian manifolds, analytical and numerical
Jacobians, and Levenberg-Marquardt damping powered by `faer`.

## Relationships

| Type | Target |
|------|--------|
| related | [DualQuaternion](/crates/oxide-geo-ops/src/solver_3d/DualQuaternion.md) |
| related | [default](/crates/oxide-geo-ops/src/solver_3d/default.md) |
| related | [default](/crates/oxide-geo-ops/src/solver_3d/default.md) |
| related | [identity](/crates/oxide-geo-ops/src/solver_3d/identity.md) |
| related | [from_translation_rotation](/crates/oxide-geo-ops/src/solver_3d/from_translation_rotation.md) |
| related | [from_translation](/crates/oxide-geo-ops/src/solver_3d/from_translation.md) |
| related | [from_axis_angle](/crates/oxide-geo-ops/src/solver_3d/from_axis_angle.md) |
| related | [translation](/crates/oxide-geo-ops/src/solver_3d/translation.md) |
| related | [transform_point](/crates/oxide-geo-ops/src/solver_3d/transform_point.md) |
| related | [transform_vector](/crates/oxide-geo-ops/src/solver_3d/transform_vector.md) |
| related | [retract](/crates/oxide-geo-ops/src/solver_3d/retract.md) |
| related | [multiply](/crates/oxide-geo-ops/src/solver_3d/multiply.md) |
| related | [identity](/crates/oxide-geo-ops/src/solver_3d/identity.md) |
| related | [from_translation_rotation](/crates/oxide-geo-ops/src/solver_3d/from_translation_rotation.md) |
| related | [from_translation](/crates/oxide-geo-ops/src/solver_3d/from_translation.md) |
| related | [from_axis_angle](/crates/oxide-geo-ops/src/solver_3d/from_axis_angle.md) |
| related | [translation](/crates/oxide-geo-ops/src/solver_3d/translation.md) |
| related | [transform_point](/crates/oxide-geo-ops/src/solver_3d/transform_point.md) |
| related | [transform_vector](/crates/oxide-geo-ops/src/solver_3d/transform_vector.md) |
| related | [retract](/crates/oxide-geo-ops/src/solver_3d/retract.md) |
| related | [multiply](/crates/oxide-geo-ops/src/solver_3d/multiply.md) |
| related | [px_dummy](/crates/oxide-geo-ops/src/solver_3d/px_dummy.md) |
| related | [Constraint3D](/crates/oxide-geo-ops/src/solver_3d/Constraint3D.md) |
| related | [SolverStatus](/crates/oxide-geo-ops/src/solver_3d/SolverStatus.md) |
| related | [SolverConfig](/crates/oxide-geo-ops/src/solver_3d/SolverConfig.md) |
| related | [default](/crates/oxide-geo-ops/src/solver_3d/default.md) |
| related | [default](/crates/oxide-geo-ops/src/solver_3d/default.md) |
| related | [ConstraintSolver3D](/crates/oxide-geo-ops/src/solver_3d/ConstraintSolver3D.md) |
| related | [new](/crates/oxide-geo-ops/src/solver_3d/new.md) |
| related | [add_body](/crates/oxide-geo-ops/src/solver_3d/add_body.md) |
| related | [add_constraint](/crates/oxide-geo-ops/src/solver_3d/add_constraint.md) |
| related | [count_equations](/crates/oxide-geo-ops/src/solver_3d/count_equations.md) |
| related | [compute_residuals](/crates/oxide-geo-ops/src/solver_3d/compute_residuals.md) |
| related | [solve](/crates/oxide-geo-ops/src/solver_3d/solve.md) |
| related | [new](/crates/oxide-geo-ops/src/solver_3d/new.md) |
| related | [add_body](/crates/oxide-geo-ops/src/solver_3d/add_body.md) |
| related | [add_constraint](/crates/oxide-geo-ops/src/solver_3d/add_constraint.md) |
| related | [count_equations](/crates/oxide-geo-ops/src/solver_3d/count_equations.md) |
| related | [compute_residuals](/crates/oxide-geo-ops/src/solver_3d/compute_residuals.md) |
| related | [solve](/crates/oxide-geo-ops/src/solver_3d/solve.md) |
| related | [test_point_coincidence_solve](/crates/oxide-geo-ops/src/solver_3d/test_point_coincidence_solve.md) |
| related | [test_distance_and_parallel_axes](/crates/oxide-geo-ops/src/solver_3d/test_distance_and_parallel_axes.md) |
| related | [faer](/_dependencies/cargo/faer.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
