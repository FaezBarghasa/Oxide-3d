---
okf_version: "0.2"
type: Function
title: get_body_position
description: "Get the current position of a body entity [x, y, z]."
resource: crates/oxide-mech/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mech"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-mech/src/lib/get_body_position
language: rust
---

# get_body_position

Get the current position of a body entity [x, y, z].

## Signature

```rust
impl MechanismWorld { pub fn get_body_position(&self, entity: EntityKey) -> Option<[f64; 3]> }
```

## Visibility

- `pub`

## Docstring

Get the current position of a body entity [x, y, z].

## Source
Lines 204–209 in `crates/oxide-mech/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-mech/src/lib.md) |
