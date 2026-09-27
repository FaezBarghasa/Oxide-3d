---
okf_version: "0.2"
type: Function
title: evaluate_nurbs_curve_homog
description: "Helper to evaluate de Boor curve returning homogeneous coordinates [wx, wy, wz, w]."
resource: crates/oxide-geo/src/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:32:18Z"
concept_id: crates/oxide-geo/src/surface/evaluate_nurbs_curve_homog
language: rust
---

# evaluate_nurbs_curve_homog

Helper to evaluate de Boor curve returning homogeneous coordinates [wx, wy, wz, w].

## Signature

```rust
fn evaluate_nurbs_curve_homog(
    degree: usize,
    control_points: &[[f64; 4]],
    knots: &[f64],
    t: f64,
) -> [f64; 4]
```

## Docstring

Helper to evaluate de Boor curve returning homogeneous coordinates [wx, wy, wz, w].

## Source
Lines 228–298 in `crates/oxide-geo/src/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-geo/src/surface.md) |
| called_by | [evaluate_nurbs_surface](/crates/oxide-geo/src/surface/evaluate_nurbs_surface.md) |
