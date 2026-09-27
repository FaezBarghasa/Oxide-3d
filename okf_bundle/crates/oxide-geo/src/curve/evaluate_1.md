---
okf_version: "0.2"
type: Function
title: evaluate
description: "Evaluate 3D point on curve at parameter `t`."
resource: crates/oxide-geo/src/curve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo/src/curve/evaluate_1
language: rust
---

# evaluate

Evaluate 3D point on curve at parameter `t`.

## Signature

```rust
pub fn evaluate(&self, t: f64) -> [f64; 3]
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Evaluate 3D point on curve at parameter `t`.
[must_use]

## Source
Lines 38–70 in `crates/oxide-geo/src/curve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [curve](/crates/oxide-geo/src/curve.md) |
| calls | [evaluate_nurbs_curve](/crates/oxide-geo/src/curve/evaluate_nurbs_curve.md) |
