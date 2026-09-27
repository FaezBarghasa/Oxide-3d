---
okf_version: "0.2"
type: Function
title: evaluate_true_position
description: Evaluate True Position (RFS) for measured center points against nominal coordinates.
resource: crates/oxide-metrology/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-metrology"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T08:58:54Z"
concept_id: crates/oxide-metrology/src/lib/evaluate_true_position
language: rust
---

# evaluate_true_position

Evaluate True Position (RFS) for measured center points against nominal coordinates.

## Signature

```rust
impl MetrologyVerifier { pub fn evaluate_true_position(
        &self,
        samples: &[InspectionPoint],
        fcf: &FeatureControlFrame,
    ) -> Vec<InspectionReport> }
```

## Visibility

- `pub`

## Docstring

Evaluate True Position (RFS) for measured center points against nominal coordinates.
True Position Zone = 2 * sqrt(dx^2 + dy^2 + dz^2).

## Source
Lines 186–210 in `crates/oxide-metrology/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-metrology/src/lib.md) |
