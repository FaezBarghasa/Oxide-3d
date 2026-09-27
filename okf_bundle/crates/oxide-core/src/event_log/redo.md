---
okf_version: "0.2"
type: Function
title: redo
description: Redo one operation by advancing the evaluation cursor.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/redo
language: rust
---

# redo

Redo one operation by advancing the evaluation cursor.

## Signature

```rust
impl EventLog { pub fn redo(&mut self) -> CoreResult<Option<&OperationRecord>> }
```

## Visibility

- `pub`

## Docstring

Redo one operation by advancing the evaluation cursor.

## Source
Lines 194–202 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
