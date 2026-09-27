---
okf_version: "0.2"
type: Function
title: connect
description: Connect output socket of source node to input socket of destination node.
resource: crates/oxide-nodes/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-nodes"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:10:40Z"
concept_id: crates/oxide-nodes/src/lib/connect_1
language: rust
---

# connect

Connect output socket of source node to input socket of destination node.

## Signature

```rust
pub fn connect(
        &mut self,
        from_node: usize,
        from_socket: usize,
        to_node: usize,
        to_socket: usize,
    ) -> Result<(), NodeError>
```

## Visibility

- `pub`

## Docstring

Connect output socket of source node to input socket of destination node.

## Source
Lines 1065–1092 in `crates/oxide-nodes/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-nodes/src/lib.md) |
