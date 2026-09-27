---
okf_version: "0.2"
type: Class
title: FeaStaticResult
description: Results of a linear static structural solve.
resource: crates/oxide-sim-fea/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim-fea"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-sim-fea/src/lib/FeaStaticResult
language: rust
---

# FeaStaticResult

Results of a linear static structural solve.

## Signature

```rust
pub struct FeaStaticResult
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Results of a linear static structural solve.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `displacements`
- `von_mises_stress`
- `max_von_mises`

## Source
Lines 36–43 in `crates/oxide-sim-fea/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-fea/src/lib.md) |
