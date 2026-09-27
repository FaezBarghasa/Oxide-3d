---
okf_version: "0.2"
type: Function
title: vectors_perpendicular
description: Check if two vectors are perpendicular within angular tolerance.
resource: crates/oxide-math/src/tolerance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:54:05Z"
concept_id: crates/oxide-math/src/tolerance/vectors_perpendicular
language: rust
---

# vectors_perpendicular

Check if two vectors are perpendicular within angular tolerance.

## Signature

```rust
impl ToleranceContext { pub fn vectors_perpendicular(&self, v1: [f64; 3], v2: [f64; 3]) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if two vectors are perpendicular within angular tolerance.
[must_use]

## Source
Lines 100–109 in `crates/oxide-math/src/tolerance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tolerance](/crates/oxide-math/src/tolerance.md) |
