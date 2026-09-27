---
okf_version: "0.2"
type: Function
title: calculate_bom_rollup
description: Recursively calculate total aggregated bill of materials roll-up for a root assembly.
resource: crates/oxide-plm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-plm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-plm/src/lib/calculate_bom_rollup_1
language: rust
---

# calculate_bom_rollup

Recursively calculate total aggregated bill of materials roll-up for a root assembly.

## Signature

```rust
pub fn calculate_bom_rollup(&self, root: ItemId) -> Vec<BomRollupEntry>
```

## Visibility

- `pub`

## Docstring

Recursively calculate total aggregated bill of materials roll-up for a root assembly.

## Source
Lines 136–154 in `crates/oxide-plm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-plm/src/lib.md) |
