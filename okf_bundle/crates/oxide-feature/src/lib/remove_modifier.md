---
okf_version: "0.2"
type: Function
title: remove_modifier
description: Remove a modifier by index.
resource: crates/oxide-feature/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-feature"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:21:18Z"
concept_id: crates/oxide-feature/src/lib/remove_modifier
language: rust
---

# remove_modifier

Remove a modifier by index.

## Signature

```rust
impl ModifierStack { pub fn remove_modifier(&mut self, index: usize) -> Option<ModifierItem> }
```

## Visibility

- `pub`

## Docstring

Remove a modifier by index.

## Source
Lines 292–298 in `crates/oxide-feature/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-feature/src/lib.md) |
