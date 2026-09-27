---
okf_version: "0.2"
type: Class
title: OperationPayload
description: Generic atomic operational delta applied to the document.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/OperationPayload
language: rust
---

# OperationPayload

Generic atomic operational delta applied to the document.

## Signature

```rust
pub enum OperationPayload
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Generic atomic operational delta applied to the document.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `entity_type`
- `data`
- `target`
- `parameter`
- `old_value`
- `new_value`
- `target`
- `snapshot`
- `feature_type`
- `params`
- `target`
- `modifier_type`
- `settings`
- `target`
- `property_path`
- `time`
- `value`
- `op_tag`
- `data`

## Source
Lines 24–84 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
