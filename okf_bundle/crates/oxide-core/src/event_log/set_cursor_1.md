---
okf_version: "0.2"
type: Function
title: set_cursor
description: "Move the rollback bar to a specific historical operation index $k \\in [0, N]$."
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/set_cursor_1
language: rust
---

# set_cursor

Move the rollback bar to a specific historical operation index $k \in [0, N]$.

## Signature

```rust
pub fn set_cursor(&mut self, new_cursor: usize) -> CoreResult<()>
```

## Visibility

- `pub`

## Docstring

Move the rollback bar to a specific historical operation index $k \in [0, N]$.

## Source
Lines 205–224 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
