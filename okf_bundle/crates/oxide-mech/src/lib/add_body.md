---
okf_version: "0.2"
type: Function
title: add_body
description: Add a rigid body into the mechanism.
resource: crates/oxide-mech/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mech"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-mech/src/lib/add_body
language: rust
---

# add_body

Add a rigid body into the mechanism.

## Signature

```rust
impl MechanismWorld { pub fn add_body(&mut self, body: &MechBody) -> RigidBodyHandle }
```

## Visibility

- `pub`

## Docstring

Add a rigid body into the mechanism.

## Source
Lines 104–136 in `crates/oxide-mech/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-mech/src/lib.md) |
