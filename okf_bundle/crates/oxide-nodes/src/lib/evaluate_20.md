---
okf_version: "0.2"
type: Function
title: evaluate
description: Topologically evaluate the entire graph and return the output sockets for each node.
resource: crates/oxide-nodes/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-nodes"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:10:40Z"
concept_id: crates/oxide-nodes/src/lib/evaluate_20
language: rust
---

# evaluate

Topologically evaluate the entire graph and return the output sockets for each node.

## Signature

```rust
impl NodeGraph { pub fn evaluate(&self) -> Result<HashMap<usize, Vec<NodeSocketValue>>, NodeError> }
```

## Visibility

- `pub`

## Docstring

Topologically evaluate the entire graph and return the output sockets for each node.

## Source
Lines 1095–1139 in `crates/oxide-nodes/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-nodes/src/lib.md) |
