---
okf_version: "0.2"
type: Function
title: check_euler_poincare
description: Validates Euler-Poincaré formula for a simple closed polyhedron (V - E + F = 2).
resource: crates/oxide-test-utils/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-test-utils"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T20:57:46Z"
concept_id: crates/oxide-test-utils/src/lib/check_euler_poincare
language: rust
---

# check_euler_poincare

Validates Euler-Poincaré formula for a simple closed polyhedron (V - E + F = 2).

## Signature

```rust
pub fn check_euler_poincare(db: &TopologyDatabase) -> bool
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Validates Euler-Poincaré formula for a simple closed polyhedron (V - E + F = 2).
[must_use]

## Source
Lines 7–15 in `crates/oxide-test-utils/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-test-utils/src/lib.md) |
