---
okf_version: "0.2"
type: Function
title: import_dxf
description: Import an ASCII DXF file into a 2D drafting database.
resource: crates/oxide-geo-io/src/cad_dxf_dwg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-io/src/cad_dxf_dwg/import_dxf_1
language: rust
---

# import_dxf

Import an ASCII DXF file into a 2D drafting database.

## Signature

```rust
pub fn import_dxf(path: P) -> Result<DraftingDatabase2D, IoFormatError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Import an ASCII DXF file into a 2D drafting database.

## Source
Lines 148–254 in `crates/oxide-geo-io/src/cad_dxf_dwg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cad_dxf_dwg](/crates/oxide-geo-io/src/cad_dxf_dwg.md) |
