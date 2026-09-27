---
okf_version: "0.2"
type: Class
title: CollabMessage
description: Collaboration sync message transferred over WebSockets.
resource: crates/oxide-collab/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-collab"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-collab/src/lib/CollabMessage
language: rust
---

# CollabMessage

Collaboration sync message transferred over WebSockets.

## Signature

```rust
pub enum CollabMessage
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Collaboration sync message transferred over WebSockets.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `username`
- `username`
- `cursor`
- `payload`

## Source
Lines 9–27 in `crates/oxide-collab/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-collab/src/lib.md) |
