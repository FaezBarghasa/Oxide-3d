---
okf_version: "0.2"
type: Function
title: evaluate_nurbs_curve
description: "De Boor algorithm for evaluating rational B-Spline / NURBS curve at parameter `t`."
resource: crates/oxide-geo/src/curve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo/src/curve/evaluate_nurbs_curve
language: rust
---

# evaluate_nurbs_curve

De Boor algorithm for evaluating rational B-Spline / NURBS curve at parameter `t`.

## Signature

```rust
fn evaluate_nurbs_curve(
    degree: usize,
    control_points: &[[f64; 4]],
    knots: &[f64],
    t: f64,
) -> [f64; 3]
```

## Docstring

De Boor algorithm for evaluating rational B-Spline / NURBS curve at parameter `t`.

## Source
Lines 91–163 in `crates/oxide-geo/src/curve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [curve](/crates/oxide-geo/src/curve.md) |
| called_by | [evaluate](/crates/oxide-geo/src/curve/evaluate.md) |
