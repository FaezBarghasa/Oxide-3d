---
okf_version: "0.2"
type: Function
title: new
description: Create a new applied operation record with generated UUIDv7.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/new
language: rust
---

# new

Create a new applied operation record with generated UUIDv7.

## Signature

```rust
impl OperationRecord { pub fn new(label: impl Into<String>, payload: OperationPayload) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new applied operation record with generated UUIDv7.
[must_use]

## Source
Lines 104–117 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
