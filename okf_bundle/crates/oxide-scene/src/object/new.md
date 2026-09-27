---
okf_version: "0.2"
type: Function
title: new
description: Create a new generic Object.
resource: crates/oxide-scene/src/object.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-scene"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:21:34Z"
concept_id: crates/oxide-scene/src/object/new
language: rust
---

# new

Create a new generic Object.

## Signature

```rust
impl ObjectProperties { pub fn new(name: impl Into<String>, data: ObjectData) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new generic Object.
[must_use]

## Source
Lines 141–155 in `crates/oxide-scene/src/object.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [object](/crates/oxide-scene/src/object.md) |
