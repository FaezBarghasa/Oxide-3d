---
okf_version: "0.2"
type: Function
title: insphere_tol
description: Tolerance-aware 3D insphere predicate.
resource: crates/oxide-math/src/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:57:42Z"
concept_id: crates/oxide-math/src/predicates/insphere_tol
language: rust
---

# insphere_tol

Tolerance-aware 3D insphere predicate.

## Signature

```rust
pub fn insphere_tol(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3], pe: [f64; 3], tol: &ToleranceContext) -> i8
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Tolerance-aware 3D insphere predicate.
[must_use]

## Source
Lines 130–139 in `crates/oxide-math/src/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-math/src/predicates.md) |
| calls | [insphere](/crates/oxide-math/src/predicates/insphere.md) |
