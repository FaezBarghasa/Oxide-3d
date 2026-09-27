---
okf_version: "0.2"
type: Function
title: new
description: Initialize a SIMP optimizer with uniform density satisfying target volume fraction.
resource: crates/oxide-sim-topopt/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim-topopt"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T22:11:55Z"
concept_id: crates/oxide-sim-topopt/src/lib/new_1
language: rust
---

# new

Initialize a SIMP optimizer with uniform density satisfying target volume fraction.

## Signature

```rust
pub fn new(nelx: usize, nely: usize, config: TopOptConfig) -> Self
```

## Visibility

- `pub`

## Docstring

Initialize a SIMP optimizer with uniform density satisfying target volume fraction.

## Source
Lines 45–53 in `crates/oxide-sim-topopt/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-topopt/src/lib.md) |
