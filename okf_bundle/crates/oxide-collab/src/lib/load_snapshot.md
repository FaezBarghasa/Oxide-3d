---
okf_version: "0.2"
type: Function
title: load_snapshot
description: Load a full document snapshot.
resource: crates/oxide-collab/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-collab"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-collab/src/lib/load_snapshot
language: rust
---

# load_snapshot

Load a full document snapshot.

## Signature

```rust
impl CollabSession { pub fn load_snapshot(data: &[u8]) -> Result<Self, String> }
```

## Visibility

- `pub`

## Docstring

Load a full document snapshot.

## Source
Lines 84–90 in `crates/oxide-collab/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-collab/src/lib.md) |
