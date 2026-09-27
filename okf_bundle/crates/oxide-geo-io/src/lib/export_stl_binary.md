---
okf_version: "0.2"
type: Function
title: export_stl_binary
description: Export a triangle mesh to binary STL format.
resource: crates/oxide-geo-io/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:45:43Z"
concept_id: crates/oxide-geo-io/src/lib/export_stl_binary
language: rust
---

# export_stl_binary

Export a triangle mesh to binary STL format.

## Signature

```rust
pub fn export_stl_binary(
    path: P,
    mesh: &TriangleMesh,
) -> Result<(), IoFormatError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Export a triangle mesh to binary STL format.

## Source
Lines 38–66 in `crates/oxide-geo-io/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-geo-io/src/lib.md) |
