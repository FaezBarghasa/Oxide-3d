---
okf_version: "0.2"
type: Function
title: add_feature
description: Add a feature node to the tree.
resource: crates/oxide-feature/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-feature"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:21:18Z"
concept_id: crates/oxide-feature/src/lib/add_feature
language: rust
---

# add_feature

Add a feature node to the tree.

## Signature

```rust
impl FeatureGraph { pub fn add_feature(&mut self, node: FeatureNode) -> NodeIndex }
```

## Visibility

- `pub`

## Docstring

Add a feature node to the tree.

## Source
Lines 336–338 in `crates/oxide-feature/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-feature/src/lib.md) |
