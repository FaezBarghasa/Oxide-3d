---
okf_version: "0.2"
type: Function
title: active_entries
description: Slice of currently active operations (up to cursor).
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/active_entries
language: rust
---

# active_entries

Slice of currently active operations (up to cursor).

## Signature

```rust
impl EventLog { pub fn active_entries(&self) -> &[OperationRecord] }
```

## Visibility

- `pub`

## Docstring

Slice of currently active operations (up to cursor).
[must_use]

## Source
Lines 179–181 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
