---
okf_version: "0.2"
type: Function
title: add_modifier
description: Add a modifier to the end of the stack.
resource: crates/oxide-feature/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-feature"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:21:18Z"
concept_id: crates/oxide-feature/src/lib/add_modifier_1
language: rust
---

# add_modifier

Add a modifier to the end of the stack.

## Signature

```rust
pub fn add_modifier(&mut self, name: impl Into<String>, kind: ModifierKind)
```

## Visibility

- `pub`

## Docstring

Add a modifier to the end of the stack.

## Source
Lines 282–289 in `crates/oxide-feature/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-feature/src/lib.md) |
