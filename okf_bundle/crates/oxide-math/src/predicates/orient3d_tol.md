---
okf_version: "0.2"
type: Function
title: orient3d_tol
description: Tolerance-aware 3D orientation predicate.
resource: crates/oxide-math/src/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:57:42Z"
concept_id: crates/oxide-math/src/predicates/orient3d_tol
language: rust
---

# orient3d_tol

Tolerance-aware 3D orientation predicate.

## Signature

```rust
pub fn orient3d_tol(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3], tol: &ToleranceContext) -> i8
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Tolerance-aware 3D orientation predicate.
Returns 1 if below, -1 if above, 0 if coplanar within tolerance.
[must_use]

## Source
Lines 59–68 in `crates/oxide-math/src/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-math/src/predicates.md) |
| calls | [orient3d](/crates/oxide-math/src/predicates/orient3d.md) |
