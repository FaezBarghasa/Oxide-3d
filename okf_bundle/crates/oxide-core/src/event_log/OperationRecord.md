---
okf_version: "0.2"
type: Class
title: OperationRecord
description: "Immutable historical entry in the document's event stream."
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/OperationRecord
language: rust
---

# OperationRecord

Immutable historical entry in the document's event stream.

## Signature

```rust
pub struct OperationRecord
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Immutable historical entry in the document's event stream.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `label`
- `payload`
- `status`
- `timestamp_ms`

## Source
Lines 88–99 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
