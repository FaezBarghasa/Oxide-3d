---
okf_version: "0.2"
type: Function
title: new
description: Create new recovery manager.
resource: crates/oxide-geo-io/src/cad_dxf_dwg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-io/src/cad_dxf_dwg/new_5
language: rust
---

# new

Create new recovery manager.

## Signature

```rust
pub fn new(autosave_dir: PathBuf, interval_minutes: u32) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create new recovery manager.
[must_use]

## Source
Lines 331–336 in `crates/oxide-geo-io/src/cad_dxf_dwg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cad_dxf_dwg](/crates/oxide-geo-io/src/cad_dxf_dwg.md) |
