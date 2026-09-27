---
okf_version: "0.2"
type: Function
title: from_magic
description: Detect DWG format version from 6-byte magic preamble.
resource: crates/oxide-geo-io/src/cad_dxf_dwg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-io/src/cad_dxf_dwg/from_magic_1
language: rust
---

# from_magic

Detect DWG format version from 6-byte magic preamble.

## Signature

```rust
pub fn from_magic(magic: &[u8; 6]) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Detect DWG format version from 6-byte magic preamble.
[must_use]

## Source
Lines 281–292 in `crates/oxide-geo-io/src/cad_dxf_dwg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cad_dxf_dwg](/crates/oxide-geo-io/src/cad_dxf_dwg.md) |
