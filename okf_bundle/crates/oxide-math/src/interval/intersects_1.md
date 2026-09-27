---
okf_version: "0.2"
type: Function
title: intersects
description: Check if two 3D intervals intersect (AABB intersection test).
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/intersects_1
language: rust
---

# intersects

Check if two 3D intervals intersect (AABB intersection test).

## Signature

```rust
pub fn intersects(&self, other: &Self) -> bool
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Check if two 3D intervals intersect (AABB intersection test).
[must_use]

## Source
Lines 124–126 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
