---
okf_version: "0.2"
type: Function
title: solve_linear_static
description: Solves a linear static stress analysis problem using Faer dense/sparse linear algebra.
resource: crates/oxide-sim-fea/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim-fea"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-sim-fea/src/lib/solve_linear_static
language: rust
---

# solve_linear_static

Solves a linear static stress analysis problem using Faer dense/sparse linear algebra.

## Signature

```rust
pub fn solve_linear_static(
    mesh: &SimulationMesh,
    material: &LinearElasticMaterial,
    bc: &BoundaryCondition,
) -> Result<FeaStaticResult, FeaError>
```

## Visibility

- `pub`

## Docstring

Solves a linear static stress analysis problem using Faer dense/sparse linear algebra.

## Source
Lines 46–164 in `crates/oxide-sim-fea/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-fea/src/lib.md) |
