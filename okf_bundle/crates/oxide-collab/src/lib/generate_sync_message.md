---
okf_version: "0.2"
type: Function
title: generate_sync_message
description: Generate an incremental sync message for a peer.
resource: crates/oxide-collab/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-collab"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-collab/src/lib/generate_sync_message
language: rust
---

# generate_sync_message

Generate an incremental sync message for a peer.

## Signature

```rust
impl CollabSession { pub fn generate_sync_message(&mut self) -> Option<Vec<u8>> }
```

## Visibility

- `pub`

## Docstring

Generate an incremental sync message for a peer.

## Source
Lines 60–65 in `crates/oxide-collab/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-collab/src/lib.md) |
