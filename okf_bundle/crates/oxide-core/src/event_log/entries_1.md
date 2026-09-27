---
okf_version: "0.2"
type: Function
title: entries
description: Slice of all operations.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/entries_1
language: rust
---

# entries

Slice of all operations.

## Signature

```rust
pub fn entries(&self) -> &[OperationRecord]
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Slice of all operations.
[must_use]

## Source
Lines 173–175 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
