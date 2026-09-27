---
okf_version: "0.2"
type: Function
title: import_obj
description: Import a triangle mesh from Wavefront OBJ format.
resource: crates/oxide-geo-io/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:45:43Z"
concept_id: crates/oxide-geo-io/src/lib/import_obj
language: rust
---

# import_obj

Import a triangle mesh from Wavefront OBJ format.

## Signature

```rust
pub fn import_obj(path: P) -> Result<TriangleMesh, IoFormatError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Import a triangle mesh from Wavefront OBJ format.

## Source
Lines 100–181 in `crates/oxide-geo-io/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-geo-io/src/lib.md) |
| called_by | [test_obj_export_and_import](/crates/oxide-geo-io/src/lib/test_obj_export_and_import.md) |
