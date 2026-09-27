---
okf_version: "0.2"
type: Function
title: dispatch
description: Execute a command through the bus.
resource: crates/oxide-core/src/bus.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-core/src/bus/dispatch
language: rust
---

# dispatch

Execute a command through the bus.

## Signature

```rust
impl CommandBus { pub fn dispatch(&self, command: OxideCommand) -> CoreResult<()> }
```

## Visibility

- `pub`

## Docstring

Execute a command through the bus.

## Source
Lines 47–51 in `crates/oxide-core/src/bus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bus](/crates/oxide-core/src/bus.md) |
