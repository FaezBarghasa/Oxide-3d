---
okf_version: "0.2"
type: Function
title: solve
description: Solve the 3D constraint system using Levenberg-Marquardt with adaptive damping.
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/solve_1
language: rust
---

# solve

Solve the 3D constraint system using Levenberg-Marquardt with adaptive damping.

## Signature

```rust
pub fn solve(&mut self, config: &SolverConfig) -> SolverStatus
```

## Visibility

- `pub`

## Docstring

Solve the 3D constraint system using Levenberg-Marquardt with adaptive damping.

## Source
Lines 437–539 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
