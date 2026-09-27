---
okf_version: "0.2"
type: Function
title: translation
description: "Extract translation vector [tx, ty, tz]."
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/translation
language: rust
---

# translation

Extract translation vector [tx, ty, tz].

## Signature

```rust
impl DualQuaternion { pub fn translation(&self) -> [f64; 3] }
```

## Visibility

- `pub`

## Docstring

Extract translation vector [tx, ty, tz].

## Source
Lines 65–81 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
