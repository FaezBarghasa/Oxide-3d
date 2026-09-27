---
okf_version: "0.2"
type: Function
title: cylinder
description: "Create a cylinder mesh given radius, height, and circumferential segments."
resource: crates/oxide-nodes/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-nodes"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:10:40Z"
concept_id: crates/oxide-nodes/src/lib/cylinder
language: rust
---

# cylinder

Create a cylinder mesh given radius, height, and circumferential segments.

## Signature

```rust
impl NodeMeshData { pub fn cylinder(radius: f32, height: f32, segments: usize) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a cylinder mesh given radius, height, and circumferential segments.

## Source
Lines 126–157 in `crates/oxide-nodes/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-nodes/src/lib.md) |
