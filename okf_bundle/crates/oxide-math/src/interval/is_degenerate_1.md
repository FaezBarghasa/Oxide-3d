---
okf_version: "0.2"
type: Function
title: is_degenerate
description: Check if the interval is degenerate (zero volume).
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/is_degenerate_1
language: rust
---

# is_degenerate

Check if the interval is degenerate (zero volume).

## Signature

```rust
pub fn is_degenerate(&self, tol: f64) -> bool
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Check if the interval is degenerate (zero volume).
[must_use]

## Source
Lines 180–182 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
