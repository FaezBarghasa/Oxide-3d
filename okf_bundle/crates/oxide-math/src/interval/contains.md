---
okf_version: "0.2"
type: Function
title: contains
description: Check if a value is contained within the interval.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/contains
language: rust
---

# contains

Check if a value is contained within the interval.

## Signature

```rust
impl Interval { pub fn contains(&self, val: f64) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if a value is contained within the interval.
[must_use]

## Source
Lines 34–36 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
