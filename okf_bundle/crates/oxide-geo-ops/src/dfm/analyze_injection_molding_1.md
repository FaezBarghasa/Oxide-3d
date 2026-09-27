---
okf_version: "0.2"
type: Function
title: analyze_injection_molding
description: Analyze mesh for Injection Molding draft angles.
resource: crates/oxide-geo-ops/src/dfm.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:43:01Z"
concept_id: crates/oxide-geo-ops/src/dfm/analyze_injection_molding_1
language: rust
---

# analyze_injection_molding

Analyze mesh for Injection Molding draft angles.

## Signature

```rust
pub fn analyze_injection_molding(mesh: &TessellatedMesh, config: &InjectionMoldingDfmConfig) -> Vec<DfmIssue>
```

## Visibility

- `pub`

## Docstring

Analyze mesh for Injection Molding draft angles.

## Source
Lines 250–317 in `crates/oxide-geo-ops/src/dfm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dfm](/crates/oxide-geo-ops/src/dfm.md) |
