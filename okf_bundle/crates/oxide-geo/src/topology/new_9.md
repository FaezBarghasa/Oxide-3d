---
okf_version: "0.2"
type: Function
title: new
description: Create a new shell from a list of face keys.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/new_9
language: rust
---

# new

Create a new shell from a list of face keys.

## Signature

```rust
pub fn new(faces: Vec<FaceKey>, is_closed: bool) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create a new shell from a list of face keys.
[must_use]

## Source
Lines 96–98 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
