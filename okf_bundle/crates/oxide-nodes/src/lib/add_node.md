---
okf_version: "0.2"
type: Function
title: add_node
description: Add a node to the graph and return its ID.
resource: crates/oxide-nodes/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-nodes"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:10:40Z"
concept_id: crates/oxide-nodes/src/lib/add_node
language: rust
---

# add_node

Add a node to the graph and return its ID.

## Signature

```rust
impl NodeGraph { pub fn add_node(&mut self, node: Arc<dyn OxideNode>) -> usize }
```

## Visibility

- `pub`

## Docstring

Add a node to the graph and return its ID.

## Source
Lines 1057–1062 in `crates/oxide-nodes/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-nodes/src/lib.md) |
