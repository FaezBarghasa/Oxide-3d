---
okf_version: "0.2"
type: Class
title: Vertex
description: Standard vertex structure uploaded to GPU vertex buffers.
resource: crates/oxide-render/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:09:07Z"
concept_id: crates/oxide-render/src/mesh/Vertex
language: rust
---

# Vertex

Standard vertex structure uploaded to GPU vertex buffers.

## Signature

```rust
pub struct Vertex
```

## Decorators

- `repr(C)`
- `derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Standard vertex structure uploaded to GPU vertex buffers.
[repr(C)]
[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable, Serialize, Deserialize)]

## Methods

- `position`
- `normal`
- `color`

## Source
Lines 9–16 in `crates/oxide-render/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/oxide-render/src/mesh.md) |
