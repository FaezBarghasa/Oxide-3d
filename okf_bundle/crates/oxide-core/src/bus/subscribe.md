---
okf_version: "0.2"
type: Function
title: subscribe
description: Subscribe to all emitted events on the bus.
resource: crates/oxide-core/src/bus.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-core/src/bus/subscribe
language: rust
---

# subscribe

Subscribe to all emitted events on the bus.

## Signature

```rust
impl CommandBus { pub fn subscribe(&self, callback: F) }
```

## Type Parameters

- `F`

## Visibility

- `pub`

## Docstring

Subscribe to all emitted events on the bus.

## Source
Lines 28–35 in `crates/oxide-core/src/bus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bus](/crates/oxide-core/src/bus.md) |
