---
okf_version: "0.2"
type: Function
title: overlaps
description: Check if two intervals overlap.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/overlaps_1
language: rust
---

# overlaps

Check if two intervals overlap.

## Signature

```rust
pub fn overlaps(&self, other: &Self) -> bool
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Check if two intervals overlap.
[must_use]

## Source
Lines 61–63 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
