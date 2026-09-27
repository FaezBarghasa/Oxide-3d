---
okf_version: "0.2"
type: Function
title: orient2d
description: Exact adaptive 2D orientation predicate using robust floating point arithmetic.
resource: crates/oxide-math/src/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:57:42Z"
concept_id: crates/oxide-math/src/predicates/orient2d
language: rust
---

# orient2d

Exact adaptive 2D orientation predicate using robust floating point arithmetic.

## Signature

```rust
pub fn orient2d(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2]) -> f64
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Exact adaptive 2D orientation predicate using robust floating point arithmetic.
Returns positive if a, b, c are counter-clockwise, negative if clockwise, zero if collinear.
[must_use]

## Source
Lines 6–12 in `crates/oxide-math/src/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-math/src/predicates.md) |
| called_by | [orient2d_tol](/crates/oxide-math/src/predicates/orient2d_tol.md) |
