---
okf_version: "0.2"
type: Function
title: new
description: Create a new mechanism simulation world with gravity vector.
resource: crates/oxide-mech/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mech"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-mech/src/lib/new
language: rust
---

# new

Create a new mechanism simulation world with gravity vector.

## Signature

```rust
impl MechanismWorld { pub fn new(gravity: [f64; 3]) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new mechanism simulation world with gravity vector.

## Source
Lines 86–101 in `crates/oxide-mech/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-mech/src/lib.md) |
