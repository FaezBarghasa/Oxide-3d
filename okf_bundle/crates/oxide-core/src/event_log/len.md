---
okf_version: "0.2"
type: Function
title: len
description: Number of total operations recorded.
resource: crates/oxide-core/src/event_log.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:04:46Z"
concept_id: crates/oxide-core/src/event_log/len
language: rust
---

# len

Number of total operations recorded.

## Signature

```rust
impl EventLog { pub fn len(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Number of total operations recorded.
[must_use]

## Source
Lines 155–157 in `crates/oxide-core/src/event_log.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event_log](/crates/oxide-core/src/event_log.md) |
