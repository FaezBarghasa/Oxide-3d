---
okf_version: "0.2"
type: Class
title: ObjectProperties
description: "Object mode transform, relations, and viewport display properties."
resource: crates/oxide-scene/src/object.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-scene"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:21:34Z"
concept_id: crates/oxide-scene/src/object/ObjectProperties
language: rust
---

# ObjectProperties

Object mode transform, relations, and viewport display properties.

## Signature

```rust
pub struct ObjectProperties
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Object mode transform, relations, and viewport display properties.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `transform`
- `delta_transform`
- `parent_id`
- `show_in_viewport`
- `show_in_render`
- `show_bounds`
- `show_name_tag`
- `show_axis`
- `in_front`
- `data`

## Source
Lines 113–136 in `crates/oxide-scene/src/object.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [object](/crates/oxide-scene/src/object.md) |
