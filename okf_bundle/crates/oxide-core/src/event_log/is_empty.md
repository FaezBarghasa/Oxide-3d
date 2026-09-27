---
okf_version: "0.2"
type: Function
title: is_empty
description: Whether the log is empty.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/is_empty
language: rust
---

# is_empty

Whether the log is empty.

## Signature

```rust
impl EventLog { pub fn is_empty(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Whether the log is empty.
[must_use]

## Source
Lines 161–163 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
