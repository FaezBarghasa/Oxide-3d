---
okf_version: "0.2"
type: Function
title: receive_sync_message
description: Receive and apply an incremental sync message from a peer.
resource: crates/oxide-collab/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-collab"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-collab/src/lib/receive_sync_message
language: rust
---

# receive_sync_message

Receive and apply an incremental sync message from a peer.

## Signature

```rust
impl CollabSession { pub fn receive_sync_message(&mut self, payload: &[u8]) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Receive and apply an incremental sync message from a peer.

## Source
Lines 68–76 in `crates/oxide-collab/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-collab/src/lib.md) |
