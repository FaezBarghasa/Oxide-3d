---
okf_version: "0.2"
type: Function
title: new
description: Create a new tolerance context with custom values.
resource: crates/oxide-math/src/tolerance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:54:05Z"
concept_id: crates/oxide-math/src/tolerance/new_1
language: rust
---

# new

Create a new tolerance context with custom values.

## Signature

```rust
pub fn new(linear: f64, angular: f64, parametric: f64) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create a new tolerance context with custom values.
[must_use]

## Source
Lines 27–33 in `crates/oxide-math/src/tolerance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tolerance](/crates/oxide-math/src/tolerance.md) |
