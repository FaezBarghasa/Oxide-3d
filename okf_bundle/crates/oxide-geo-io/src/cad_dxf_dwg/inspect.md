---
okf_version: "0.2"
type: Function
title: inspect
description: Inspect DWG magic header bytes.
resource: crates/oxide-geo-io/src/cad_dxf_dwg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-io/src/cad_dxf_dwg/inspect
language: rust
---

# inspect

Inspect DWG magic header bytes.

## Signature

```rust
impl DwgSniffer { pub fn inspect(path: P) -> Result<DwgVersion, IoFormatError> }
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Inspect DWG magic header bytes.

## Source
Lines 301–307 in `crates/oxide-geo-io/src/cad_dxf_dwg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cad_dxf_dwg](/crates/oxide-geo-io/src/cad_dxf_dwg.md) |
