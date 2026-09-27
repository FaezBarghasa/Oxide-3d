---
okf_version: "0.2"
type: Function
title: analyze_additive
description: Analyze mesh for 3D Printing / Additive issues (overhangs requiring support structures).
resource: crates/oxide-geo-ops/src/dfm.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:43:01Z"
concept_id: crates/oxide-geo-ops/src/dfm/analyze_additive_1
language: rust
---

# analyze_additive

Analyze mesh for 3D Printing / Additive issues (overhangs requiring support structures).

## Signature

```rust
pub fn analyze_additive(mesh: &TessellatedMesh, config: &AdditiveDfmConfig) -> Vec<DfmIssue>
```

## Visibility

- `pub`

## Docstring

Analyze mesh for 3D Printing / Additive issues (overhangs requiring support structures).

## Source
Lines 110–183 in `crates/oxide-geo-ops/src/dfm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dfm](/crates/oxide-geo-ops/src/dfm.md) |
