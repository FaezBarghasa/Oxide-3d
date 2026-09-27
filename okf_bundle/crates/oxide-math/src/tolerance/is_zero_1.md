---
okf_version: "0.2"
type: Function
title: is_zero
description: Check if a value is effectively zero within linear tolerance.
resource: crates/oxide-math/src/tolerance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:54:05Z"
concept_id: crates/oxide-math/src/tolerance/is_zero_1
language: rust
---

# is_zero

Check if a value is effectively zero within linear tolerance.

## Signature

```rust
pub fn is_zero(&self, val: f64) -> bool
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Check if a value is effectively zero within linear tolerance.
[must_use]

## Source
Lines 81–83 in `crates/oxide-math/src/tolerance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tolerance](/crates/oxide-math/src/tolerance.md) |
