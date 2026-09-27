---
okf_version: "0.2"
type: Function
title: orient3d
description: Exact adaptive 3D orientation predicate using robust floating point arithmetic.
resource: crates/oxide-math/src/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:57:42Z"
concept_id: crates/oxide-math/src/predicates/orient3d
language: rust
---

# orient3d

Exact adaptive 3D orientation predicate using robust floating point arithmetic.

## Signature

```rust
pub fn orient3d(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3]) -> f64
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Exact adaptive 3D orientation predicate using robust floating point arithmetic.
Returns positive if pd lies below plane pa-pb-pc, negative if above, zero if coplanar.
[must_use]

## Source
Lines 31–54 in `crates/oxide-math/src/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-math/src/predicates.md) |
| called_by | [orient3d_tol](/crates/oxide-math/src/predicates/orient3d_tol.md) |
