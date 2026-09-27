---
okf_version: "0.2"
type: Function
title: new
description: Create a new interval with bounds check.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/new
language: rust
---

# new

Create a new interval with bounds check.

## Signature

```rust
impl Interval { pub fn new(a: f64, b: f64) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new interval with bounds check.
[must_use]

## Source
Lines 15–21 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
