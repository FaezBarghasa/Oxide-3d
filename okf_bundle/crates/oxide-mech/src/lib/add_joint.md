---
okf_version: "0.2"
type: Function
title: add_joint
description: Connect two bodies with a kinematic joint.
resource: crates/oxide-mech/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mech"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-mech/src/lib/add_joint
language: rust
---

# add_joint

Connect two bodies with a kinematic joint.

## Signature

```rust
impl MechanismWorld { pub fn add_joint(
        &mut self,
        entity1: EntityKey,
        entity2: EntityKey,
        joint_kind: JointKind,
    ) -> Option<ImpulseJointHandle> }
```

## Visibility

- `pub`

## Docstring

Connect two bodies with a kinematic joint.

## Source
Lines 139–179 in `crates/oxide-mech/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-mech/src/lib.md) |
