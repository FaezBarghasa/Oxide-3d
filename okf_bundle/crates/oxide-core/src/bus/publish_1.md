---
okf_version: "0.2"
type: Function
title: publish
description: Emit an event to all subscribers.
resource: crates/oxide-core/src/bus.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-core/src/bus/publish_1
language: rust
---

# publish

Emit an event to all subscribers.

## Signature

```rust
pub fn publish(&self, event: &OxideEvent)
```

## Visibility

- `pub`

## Docstring

Emit an event to all subscribers.

## Source
Lines 38–44 in `crates/oxide-core/src/bus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bus](/crates/oxide-core/src/bus.md) |
