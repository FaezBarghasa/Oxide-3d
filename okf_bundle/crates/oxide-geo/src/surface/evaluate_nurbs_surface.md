---
okf_version: "0.2"
type: Function
title: evaluate_nurbs_surface
description: Tensor-product de Boor evaluation for bivariate NURBS surface.
resource: crates/oxide-geo/src/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:32:18Z"
concept_id: crates/oxide-geo/src/surface/evaluate_nurbs_surface
language: rust
---

# evaluate_nurbs_surface

Tensor-product de Boor evaluation for bivariate NURBS surface.

## Signature

```rust
fn evaluate_nurbs_surface(
    degrees: (usize, usize),
    control_points: &[Vec<[f64; 4]>],
    knots_u: &[f64],
    knots_v: &[f64],
    u: f64,
    v: f64,
) -> [f64; 3]
```

## Docstring

Tensor-product de Boor evaluation for bivariate NURBS surface.

## Source
Lines 189–225 in `crates/oxide-geo/src/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-geo/src/surface.md) |
| calls | [evaluate_nurbs_curve_homog](/crates/oxide-geo/src/surface/evaluate_nurbs_curve_homog.md) |
| called_by | [evaluate](/crates/oxide-geo/src/surface/evaluate.md) |
