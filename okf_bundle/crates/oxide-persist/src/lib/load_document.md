---
okf_version: "0.2"
type: Function
title: load_document
description: "Load an `.oxd` document from a compressed binary file."
resource: crates/oxide-persist/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-persist"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:33:51Z"
concept_id: crates/oxide-persist/src/lib/load_document
language: rust
---

# load_document

Load an `.oxd` document from a compressed binary file.

## Signature

```rust
pub fn load_document(path: P) -> Result<OxdDocument<T>, PersistError>
```

## Type Parameters

- `P: AsRef<Path>`
- `T: for<'de> Deserialize<'de`

## Visibility

- `pub`

## Docstring

Load an `.oxd` document from a compressed binary file.

## Source
Lines 77–89 in `crates/oxide-persist/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-persist/src/lib.md) |
