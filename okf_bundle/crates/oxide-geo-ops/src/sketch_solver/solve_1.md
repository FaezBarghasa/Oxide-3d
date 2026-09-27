---
okf_version: "0.2"
type: Function
title: solve
description: Solve the sketch constraints using damped Newton-Raphson iteration.
resource: crates/oxide-geo-ops/src/sketch_solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/sketch_solver/solve_1
language: rust
---

# solve

Solve the sketch constraints using damped Newton-Raphson iteration.

## Signature

```rust
pub fn solve(&mut self, max_iterations: usize, tolerance: f64) -> Result<usize, String>
```

## Visibility

- `pub`

## Docstring

Solve the sketch constraints using damped Newton-Raphson iteration.

## Source
Lines 109–225 in `crates/oxide-geo-ops/src/sketch_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_solver](/crates/oxide-geo-ops/src/sketch_solver.md) |
