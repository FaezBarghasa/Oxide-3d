---
okf_version: "0.2"
type: Function
title: analyze_cnc_milling
description: Analyze mesh for CNC Milling accessibility and pocket issues.
resource: crates/oxide-geo-ops/src/dfm.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:43:01Z"
concept_id: crates/oxide-geo-ops/src/dfm/analyze_cnc_milling
language: rust
---

# analyze_cnc_milling

Analyze mesh for CNC Milling accessibility and pocket issues.

## Signature

```rust
impl DfmEngine { pub fn analyze_cnc_milling(mesh: &TessellatedMesh, config: &CncMillingDfmConfig) -> Vec<DfmIssue> }
```

## Visibility

- `pub`

## Docstring

Analyze mesh for CNC Milling accessibility and pocket issues.

## Source
Lines 186–247 in `crates/oxide-geo-ops/src/dfm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dfm](/crates/oxide-geo-ops/src/dfm.md) |
