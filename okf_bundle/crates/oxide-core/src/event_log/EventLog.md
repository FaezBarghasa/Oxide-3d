---
okf_version: "0.2"
type: Class
title: EventLog
description: Event-Sourced Document Operation Log.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/EventLog
language: rust
---

# EventLog

Event-Sourced Document Operation Log.

## Signature

```rust
pub struct EventLog
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Event-Sourced Document Operation Log.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `entries`
- `cursor`

## Source
Lines 122–128 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
