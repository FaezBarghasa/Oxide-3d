---
okf_version: "0.2"
type: Class
title: Uniforms
description: Uniform buffer data uploaded per frame.
resource: crates/oxide-render/src/pipeline.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:42:20Z"
concept_id: crates/oxide-render/src/pipeline/Uniforms
language: rust
---

# Uniforms

Uniform buffer data uploaded per frame.

## Signature

```rust
pub struct Uniforms
```

## Decorators

- `repr(C)`
- `derive(Debug, Clone, Copy, Pod, Zeroable)`

## Visibility

- `pub`

## Docstring

Uniform buffer data uploaded per frame.
[repr(C)]
[derive(Debug, Clone, Copy, Pod, Zeroable)]

## Methods

- `view_proj`
- `light_dir`

## Source
Lines 12–17 in `crates/oxide-render/src/pipeline.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pipeline](/crates/oxide-render/src/pipeline.md) |
