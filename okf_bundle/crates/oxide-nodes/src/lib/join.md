---
okf_version: "0.2"
type: Function
title: join
description: Join another mesh into this mesh.
resource: crates/oxide-nodes/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-nodes"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:10:40Z"
concept_id: crates/oxide-nodes/src/lib/join
language: rust
---

# join

Join another mesh into this mesh.

## Signature

```rust
impl NodeMeshData { pub fn join(&mut self, other: &Self) }
```

## Visibility

- `pub`

## Docstring

Join another mesh into this mesh.

## Source
Lines 358–365 in `crates/oxide-nodes/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-nodes/src/lib.md) |
