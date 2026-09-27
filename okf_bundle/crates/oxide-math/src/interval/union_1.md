---
okf_version: "0.2"
type: Function
title: union
description: Compute the union of two intervals.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/union_1
language: rust
---

# union

Compute the union of two intervals.

## Signature

```rust
pub fn union(&self, other: &Self) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Compute the union of two intervals.
[must_use]

## Source
Lines 52–57 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
