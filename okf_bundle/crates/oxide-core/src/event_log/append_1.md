---
okf_version: "0.2"
type: Function
title: append
description: "Appends and applies a new operation to the log, truncating any rolled-back redo history."
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/append_1
language: rust
---

# append

Appends and applies a new operation to the log, truncating any rolled-back redo history.

## Signature

```rust
pub fn append(&mut self, label: impl Into<String>, payload: OperationPayload) -> OperationId
```

## Visibility

- `pub`

## Docstring

Appends and applies a new operation to the log, truncating any rolled-back redo history.

## Source
Lines 141–151 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
