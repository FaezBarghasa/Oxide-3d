---
okf_version: "0.2"
type: Class
title: ObjectData
description: Type of scene object entity in 3D Object Mode.
resource: crates/oxide-scene/src/object.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-scene"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:21:34Z"
concept_id: crates/oxide-scene/src/object/ObjectData
language: rust
---

# ObjectData

Type of scene object entity in 3D Object Mode.

## Signature

```rust
pub enum ObjectData
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Type of scene object entity in 3D Object Mode.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `name`
- `curve_type`
- `surface_type`
- `element_type`
- `stiffness`
- `body`
- `size`
- `vdb_path`
- `count`
- `display_type`
- `size`
- `kind`
- `power_watts`
- `color`
- `focal_length_mm`
- `sensor_width_mm`
- `fov_deg`

## Source
Lines 8–73 in `crates/oxide-scene/src/object.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [object](/crates/oxide-scene/src/object.md) |
