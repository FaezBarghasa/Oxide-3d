---
okf_version: "0.2"
type: Function
title: transform
description: Transform mesh vertices by translation offset and uniform scale.
resource: crates/oxide-nodes/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-nodes"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:10:40Z"
concept_id: crates/oxide-nodes/src/lib/transform
language: rust
---

# transform

Transform mesh vertices by translation offset and uniform scale.

## Signature

```rust
impl NodeMeshData { pub fn transform(&mut self, translation: [f32; 3], scale: [f32; 3]) }
```

## Visibility

- `pub`

## Docstring

Transform mesh vertices by translation offset and uniform scale.

## Source
Lines 349–355 in `crates/oxide-nodes/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-nodes/src/lib.md) |
