---
okf_version: "0.2"
type: Function
title: from_translation
description: "Construct from pure translation [tx, ty, tz]."
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/from_translation
language: rust
---

# from_translation

Construct from pure translation [tx, ty, tz].

## Signature

```rust
impl DualQuaternion { pub fn from_translation(t: [f64; 3]) -> Self }
```

## Visibility

- `pub`

## Docstring

Construct from pure translation [tx, ty, tz].

## Source
Lines 46–48 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
