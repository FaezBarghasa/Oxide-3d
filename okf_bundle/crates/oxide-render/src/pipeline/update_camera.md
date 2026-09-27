---
okf_version: "0.2"
type: Function
title: update_camera
description: Update camera uniforms on the GPU.
resource: crates/oxide-render/src/pipeline.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:42:20Z"
concept_id: crates/oxide-render/src/pipeline/update_camera
language: rust
---

# update_camera

Update camera uniforms on the GPU.

## Signature

```rust
impl RenderPipelineManager { pub fn update_camera(&self, queue: &wgpu::Queue, camera: &Camera) }
```

## Visibility

- `pub`

## Docstring

Update camera uniforms on the GPU.

## Source
Lines 213–219 in `crates/oxide-render/src/pipeline.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pipeline](/crates/oxide-render/src/pipeline.md) |
