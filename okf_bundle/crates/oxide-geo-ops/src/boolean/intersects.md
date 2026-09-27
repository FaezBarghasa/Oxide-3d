---
okf_version: "0.2"
type: Function
title: intersects
description: Check if two bounding boxes intersect with tolerance margin.
resource: crates/oxide-geo-ops/src/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/boolean/intersects
language: rust
---

# intersects

Check if two bounding boxes intersect with tolerance margin.

## Signature

```rust
impl BoundingBox3d { pub fn intersects(&self, other: &Self, tol: f64) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if two bounding boxes intersect with tolerance margin.

## Source
Lines 48–55 in `crates/oxide-geo-ops/src/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-geo-ops/src/boolean.md) |
