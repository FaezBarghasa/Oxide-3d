---
okf_version: "0.2"
type: Function
title: save_document
description: "Save an `.oxd` document to a compressed binary file."
resource: crates/oxide-persist/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-persist"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:33:51Z"
concept_id: crates/oxide-persist/src/lib/save_document
language: rust
---

# save_document

Save an `.oxd` document to a compressed binary file.

## Signature

```rust
pub fn save_document(path: P, doc: &OxdDocument<T>) -> Result<(), PersistError>
```

## Type Parameters

- `P: AsRef<Path>`
- `T: Serialize`

## Visibility

- `pub`

## Docstring

Save an `.oxd` document to a compressed binary file.

## Source
Lines 64–74 in `crates/oxide-persist/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-persist/src/lib.md) |
