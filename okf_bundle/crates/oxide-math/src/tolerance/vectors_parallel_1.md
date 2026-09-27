---
okf_version: "0.2"
type: Function
title: vectors_parallel
description: Check if two vectors are parallel within angular tolerance.
resource: crates/oxide-math/src/tolerance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:54:05Z"
concept_id: crates/oxide-math/src/tolerance/vectors_parallel_1
language: rust
---

# vectors_parallel

Check if two vectors are parallel within angular tolerance.

## Signature

```rust
pub fn vectors_parallel(&self, v1: [f64; 3], v2: [f64; 3]) -> bool
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Check if two vectors are parallel within angular tolerance.
[must_use]

## Source
Lines 87–96 in `crates/oxide-math/src/tolerance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tolerance](/crates/oxide-math/src/tolerance.md) |
