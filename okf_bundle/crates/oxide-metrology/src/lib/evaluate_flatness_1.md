---
okf_version: "0.2"
type: Function
title: evaluate_flatness
description: Evaluate Flatness of a point cloud against best-fit plane.
resource: crates/oxide-metrology/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-metrology"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T08:58:54Z"
concept_id: crates/oxide-metrology/src/lib/evaluate_flatness_1
language: rust
---

# evaluate_flatness

Evaluate Flatness of a point cloud against best-fit plane.

## Signature

```rust
pub fn evaluate_flatness(
        &self,
        points: &[[f64; 3]],
        fcf: &FeatureControlFrame,
    ) -> InspectionReport
```

## Visibility

- `pub`

## Docstring

Evaluate Flatness of a point cloud against best-fit plane.
Flatness is the minimum distance between two parallel planes containing all points.

## Source
Lines 104–182 in `crates/oxide-metrology/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-metrology/src/lib.md) |
