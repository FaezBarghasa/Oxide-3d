---
okf_version: "0.2"
type: Module
title: event_log
description: Event-Sourced Document and Temporal Operation Log for Oxide-3D.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log
language: rust
---

# event_log

Event-Sourced Document and Temporal Operation Log for Oxide-3D.

## Docstring

Event-Sourced Document and Temporal Operation Log for Oxide-3D.

Provides an immutable, append-only operation log with millisecond-accurate UUIDv7 keys.
Supports infinite undo/redo, rollback bar sliding (CAD feature rollback / DCC modifier evaluation),
branching revisions, and state snapshots.

## Relationships

| Type | Target |
|------|--------|
| related | [OperationStatus](/crates/oxide-core/src/event_log/OperationStatus.md) |
| related | [OperationPayload](/crates/oxide-core/src/event_log/OperationPayload.md) |
| related | [OperationRecord](/crates/oxide-core/src/event_log/OperationRecord.md) |
| related | [new](/crates/oxide-core/src/event_log/new.md) |
| related | [new](/crates/oxide-core/src/event_log/new.md) |
| related | [EventLog](/crates/oxide-core/src/event_log/EventLog.md) |
| related | [new](/crates/oxide-core/src/event_log/new.md) |
| related | [append](/crates/oxide-core/src/event_log/append.md) |
| related | [len](/crates/oxide-core/src/event_log/len.md) |
| related | [is_empty](/crates/oxide-core/src/event_log/is_empty.md) |
| related | [cursor](/crates/oxide-core/src/event_log/cursor.md) |
| related | [entries](/crates/oxide-core/src/event_log/entries.md) |
| related | [active_entries](/crates/oxide-core/src/event_log/active_entries.md) |
| related | [undo](/crates/oxide-core/src/event_log/undo.md) |
| related | [redo](/crates/oxide-core/src/event_log/redo.md) |
| related | [set_cursor](/crates/oxide-core/src/event_log/set_cursor.md) |
| related | [new](/crates/oxide-core/src/event_log/new.md) |
| related | [append](/crates/oxide-core/src/event_log/append.md) |
| related | [len](/crates/oxide-core/src/event_log/len.md) |
| related | [is_empty](/crates/oxide-core/src/event_log/is_empty.md) |
| related | [cursor](/crates/oxide-core/src/event_log/cursor.md) |
| related | [entries](/crates/oxide-core/src/event_log/entries.md) |
| related | [active_entries](/crates/oxide-core/src/event_log/active_entries.md) |
| related | [undo](/crates/oxide-core/src/event_log/undo.md) |
| related | [redo](/crates/oxide-core/src/event_log/redo.md) |
| related | [set_cursor](/crates/oxide-core/src/event_log/set_cursor.md) |
| related | [test_event_log_append_undo_redo](/crates/oxide-core/src/event_log/test_event_log_append_undo_redo.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
