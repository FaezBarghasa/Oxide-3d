---
okf_version: "0.2"
type: Function
title: new
description: Create a new native geometry kernel wrapping an existing or empty database.
resource: crates/oxide-geo-ops/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/kernel/new
language: rust
---

# new

Create a new native geometry kernel wrapping an existing or empty database.

## Signature

```rust
impl NativeGeometryKernel { pub fn new(db: TopologyDatabase) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new native geometry kernel wrapping an existing or empty database.

## Source
Lines 76–78 in `crates/oxide-geo-ops/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/oxide-geo-ops/src/kernel.md) |
