---
okf_version: "0.2"
type: Function
title: add_shell
description: Add a shell of connected faces.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/add_shell
language: rust
---

# add_shell

Add a shell of connected faces.

## Signature

```rust
impl TopologyDatabase { pub fn add_shell(&mut self, faces: Vec<FaceKey>, is_closed: bool) -> ShellKey }
```

## Visibility

- `pub`

## Docstring

Add a shell of connected faces.

## Source
Lines 171–173 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
