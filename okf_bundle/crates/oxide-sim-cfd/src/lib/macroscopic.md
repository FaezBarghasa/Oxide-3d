---
okf_version: "0.2"
type: Function
title: macroscopic
description: "Calculate macroscopic density rho and velocity (ux, uy) for cell (x, y)."
resource: crates/oxide-sim-cfd/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim-cfd"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-sim-cfd/src/lib/macroscopic
language: rust
---

# macroscopic

Calculate macroscopic density rho and velocity (ux, uy) for cell (x, y).

## Signature

```rust
impl LbmSolver2D { pub fn macroscopic(&self, x: usize, y: usize) -> (f64, [f64; 2]) }
```

## Visibility

- `pub`

## Docstring

Calculate macroscopic density rho and velocity (ux, uy) for cell (x, y).

## Source
Lines 93–112 in `crates/oxide-sim-cfd/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-cfd/src/lib.md) |
