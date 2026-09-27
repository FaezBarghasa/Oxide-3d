---
okf_version: "0.2"
type: Function
title: save_manifest
description: "Save an `.oxd` manifest to a JSON file path."
resource: crates/oxide-persist/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-persist"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:33:51Z"
concept_id: crates/oxide-persist/src/lib/save_manifest
language: rust
---

# save_manifest

Save an `.oxd` manifest to a JSON file path.

## Signature

```rust
pub fn save_manifest(path: P, manifest: &OxdManifest) -> Result<(), PersistError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Save an `.oxd` manifest to a JSON file path.

## Source
Lines 92–97 in `crates/oxide-persist/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-persist/src/lib.md) |
