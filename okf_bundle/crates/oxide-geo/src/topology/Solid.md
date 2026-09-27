---
okf_version: "0.2"
type: Class
title: Solid
description: 3D B-Rep Solid bounded by outer shell and optional internal cavity shells.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/Solid
language: rust
---

# Solid

3D B-Rep Solid bounded by outer shell and optional internal cavity shells.

## Signature

```rust
pub struct Solid
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

3D B-Rep Solid bounded by outer shell and optional internal cavity shells.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `outer_shell`
- `void_shells`

## Source
Lines 103–108 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
