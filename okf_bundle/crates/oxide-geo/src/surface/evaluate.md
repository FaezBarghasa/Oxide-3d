---
okf_version: "0.2"
type: Function
title: evaluate
description: "Evaluate 3D point on surface at parameters `(u, v)`."
resource: crates/oxide-geo/src/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:32:18Z"
concept_id: crates/oxide-geo/src/surface/evaluate
language: rust
---

# evaluate

Evaluate 3D point on surface at parameters `(u, v)`.

## Signature

```rust
impl Surface3d { pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] }
```

## Visibility

- `pub`

## Docstring

Evaluate 3D point on surface at parameters `(u, v)`.
[must_use]

## Source
Lines 67–165 in `crates/oxide-geo/src/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-geo/src/surface.md) |
| calls | [evaluate_nurbs_surface](/crates/oxide-geo/src/surface/evaluate_nurbs_surface.md) |
