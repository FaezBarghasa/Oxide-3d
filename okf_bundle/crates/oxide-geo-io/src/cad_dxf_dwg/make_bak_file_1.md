---
okf_version: "0.2"
type: Function
title: make_bak_file
description: "Save backup copy (`.bak`)."
resource: crates/oxide-geo-io/src/cad_dxf_dwg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-io/src/cad_dxf_dwg/make_bak_file_1
language: rust
---

# make_bak_file

Save backup copy (`.bak`).

## Signature

```rust
pub fn make_bak_file(&self, drawing_path: P) -> Result<PathBuf, IoFormatError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Save backup copy (`.bak`).

## Source
Lines 339–346 in `crates/oxide-geo-io/src/cad_dxf_dwg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cad_dxf_dwg](/crates/oxide-geo-io/src/cad_dxf_dwg.md) |
