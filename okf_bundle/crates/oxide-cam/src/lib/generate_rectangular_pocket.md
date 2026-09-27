---
okf_version: "0.2"
type: Function
title: generate_rectangular_pocket
description: Generate a multi-pass zigzag pocketing toolpath for a rectangular boundary.
resource: crates/oxide-cam/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cam"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:51:21Z"
concept_id: crates/oxide-cam/src/lib/generate_rectangular_pocket
language: rust
---

# generate_rectangular_pocket

Generate a multi-pass zigzag pocketing toolpath for a rectangular boundary.

## Signature

```rust
impl ToolpathGenerator { pub fn generate_rectangular_pocket(
        &self,
        op: &PocketOperation,
        min_pt: [f64; 2],
        max_pt: [f64; 2],
    ) -> Vec<ToolpathPoint> }
```

## Visibility

- `pub`

## Docstring

Generate a multi-pass zigzag pocketing toolpath for a rectangular boundary.

## Source
Lines 84–166 in `crates/oxide-cam/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-cam/src/lib.md) |
