---
okf_version: "0.2"
type: Function
title: angles_equal
description: Check if two angles are equal within angular tolerance.
resource: crates/oxide-math/src/tolerance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:54:05Z"
concept_id: crates/oxide-math/src/tolerance/angles_equal
language: rust
---

# angles_equal

Check if two angles are equal within angular tolerance.

## Signature

```rust
impl ToleranceContext { pub fn angles_equal(&self, a1: f64, a2: f64) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if two angles are equal within angular tolerance.
[must_use]

## Source
Lines 74–77 in `crates/oxide-math/src/tolerance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tolerance](/crates/oxide-math/src/tolerance.md) |
