---
okf_version: "0.2"
type: Function
title: cursor
description: Current evaluation cursor position.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/cursor_1
language: rust
---

# cursor

Current evaluation cursor position.

## Signature

```rust
pub fn cursor(&self) -> usize
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Current evaluation cursor position.
[must_use]

## Source
Lines 167–169 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
