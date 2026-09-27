---
okf_version: "0.2"
type: Function
title: new
description: Create a new 2D Lattice Boltzmann fluid solver.
resource: crates/oxide-sim-cfd/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim-cfd"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-sim-cfd/src/lib/new
language: rust
---

# new

Create a new 2D Lattice Boltzmann fluid solver.

## Signature

```rust
impl LbmSolver2D { pub fn new(nx: usize, ny: usize, tau: f64, inflow_velocity: [f64; 2]) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new 2D Lattice Boltzmann fluid solver.

## Source
Lines 60–75 in `crates/oxide-sim-cfd/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-cfd/src/lib.md) |
