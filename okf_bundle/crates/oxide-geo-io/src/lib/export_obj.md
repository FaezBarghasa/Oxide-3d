---
okf_version: "0.2"
type: Function
title: export_obj
description: Export a triangle mesh to Wavefront OBJ format.
resource: crates/oxide-geo-io/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:45:43Z"
concept_id: crates/oxide-geo-io/src/lib/export_obj
language: rust
---

# export_obj

Export a triangle mesh to Wavefront OBJ format.

## Signature

```rust
pub fn export_obj(path: P, mesh: &TriangleMesh) -> Result<(), IoFormatError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Export a triangle mesh to Wavefront OBJ format.

## Source
Lines 69–97 in `crates/oxide-geo-io/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-geo-io/src/lib.md) |
| called_by | [test_obj_export_and_import](/crates/oxide-geo-io/src/lib/test_obj_export_and_import.md) |
