---
okf_version: "0.2"
type: Function
title: orient2d_tol
description: Tolerance-aware 2D orientation predicate.
resource: crates/oxide-math/src/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:57:42Z"
concept_id: crates/oxide-math/src/predicates/orient2d_tol
language: rust
---

# orient2d_tol

Tolerance-aware 2D orientation predicate.

## Signature

```rust
pub fn orient2d_tol(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2], tol: &ToleranceContext) -> i8
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Tolerance-aware 2D orientation predicate.
Returns 1 if CCW, -1 if CW, 0 if collinear within tolerance.
[must_use]

## Source
Lines 17–26 in `crates/oxide-math/src/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-math/src/predicates.md) |
| calls | [orient2d](/crates/oxide-math/src/predicates/orient2d.md) |
