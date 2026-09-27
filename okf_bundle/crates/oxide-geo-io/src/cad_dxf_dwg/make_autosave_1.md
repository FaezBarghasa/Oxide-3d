---
okf_version: "0.2"
type: Function
title: make_autosave
description: "Save temporary autosave copy (`.sv$`)."
resource: crates/oxide-geo-io/src/cad_dxf_dwg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-io/src/cad_dxf_dwg/make_autosave_1
language: rust
---

# make_autosave

Save temporary autosave copy (`.sv$`).

## Signature

```rust
pub fn make_autosave(
        &self,
        drawing_path: P,
        db: &DraftingDatabase2D,
    ) -> Result<PathBuf, IoFormatError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Save temporary autosave copy (`.sv$`).

## Source
Lines 349–364 in `crates/oxide-geo-io/src/cad_dxf_dwg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cad_dxf_dwg](/crates/oxide-geo-io/src/cad_dxf_dwg.md) |
