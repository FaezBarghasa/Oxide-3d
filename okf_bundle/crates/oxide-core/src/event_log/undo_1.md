---
okf_version: "0.2"
type: Function
title: undo
description: Undo one operation by stepping the evaluation cursor back.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/undo_1
language: rust
---

# undo

Undo one operation by stepping the evaluation cursor back.

## Signature

```rust
pub fn undo(&mut self) -> CoreResult<Option<&OperationRecord>>
```

## Visibility

- `pub`

## Docstring

Undo one operation by stepping the evaluation cursor back.

## Source
Lines 184–191 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
