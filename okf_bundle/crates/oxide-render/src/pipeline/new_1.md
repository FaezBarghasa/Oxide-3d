---
okf_version: "0.2"
type: Function
title: new
description: Create a new render pipeline manager for a specific surface target format.
resource: crates/oxide-render/src/pipeline.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:42:20Z"
concept_id: crates/oxide-render/src/pipeline/new_1
language: rust
---

# new

Create a new render pipeline manager for a specific surface target format.

## Signature

```rust
pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self
```

## Visibility

- `pub`

## Docstring

Create a new render pipeline manager for a specific surface target format.

## Source
Lines 121–210 in `crates/oxide-render/src/pipeline.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pipeline](/crates/oxide-render/src/pipeline.md) |
