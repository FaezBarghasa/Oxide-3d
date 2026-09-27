---
okf_version: "0.2"
type: Function
title: add_solid
description: Add a solid bounded by a shell.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/add_solid
language: rust
---

# add_solid

Add a solid bounded by a shell.

## Signature

```rust
impl TopologyDatabase { pub fn add_solid(&mut self, outer_shell: ShellKey) -> SolidKey }
```

## Visibility

- `pub`

## Docstring

Add a solid bounded by a shell.

## Source
Lines 176–178 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
