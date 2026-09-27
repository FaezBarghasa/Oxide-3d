---
okf_version: "0.2"
type: Function
title: add_bom_child
description: Add a child component dependency into an assembly BOM.
resource: crates/oxide-plm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-plm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-plm/src/lib/add_bom_child_1
language: rust
---

# add_bom_child

Add a child component dependency into an assembly BOM.

## Signature

```rust
pub fn add_bom_child(&mut self, parent: ItemId, child: ItemId, quantity: f64)
```

## Visibility

- `pub`

## Docstring

Add a child component dependency into an assembly BOM.

## Source
Lines 123–133 in `crates/oxide-plm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-plm/src/lib.md) |
