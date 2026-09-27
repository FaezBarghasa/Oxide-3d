---
okf_version: "0.2"
type: Function
title: export_dxf
description: Export 2D drafting database to ASCII DXF file.
resource: crates/oxide-geo-io/src/cad_dxf_dwg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-io/src/cad_dxf_dwg/export_dxf
language: rust
---

# export_dxf

Export 2D drafting database to ASCII DXF file.

## Signature

```rust
impl DxfCodec { pub fn export_dxf(
        &self,
        path: P,
        db: &DraftingDatabase2D,
    ) -> Result<(), IoFormatError> }
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Export 2D drafting database to ASCII DXF file.

## Source
Lines 55–145 in `crates/oxide-geo-io/src/cad_dxf_dwg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cad_dxf_dwg](/crates/oxide-geo-io/src/cad_dxf_dwg.md) |
