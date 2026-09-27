---
okf_version: "0.2"
type: Class
title: LbmSolver2D
description: 2D/3D Lattice Boltzmann (D2Q9) fluid dynamics simulator.
resource: crates/oxide-sim-cfd/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim-cfd"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-sim-cfd/src/lib/LbmSolver2D
language: rust
---

# LbmSolver2D

2D/3D Lattice Boltzmann (D2Q9) fluid dynamics simulator.

## Signature

```rust
pub struct LbmSolver2D
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

2D/3D Lattice Boltzmann (D2Q9) fluid dynamics simulator.
[derive(Debug, Clone)]

## Methods

- `dims`
- `tau`
- `f`
- `f_temp`
- `solid_mask`
- `inflow_velocity`

## Source
Lines 25–38 in `crates/oxide-sim-cfd/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-cfd/src/lib.md) |
