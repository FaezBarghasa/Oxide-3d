---
okf_version: "0.2"
type: Function
title: load_from_path
description: Load settings from a specific path.
resource: crates/oxide-settings/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-settings"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:34:16Z"
concept_id: crates/oxide-settings/src/lib/load_from_path_1
language: rust
---

# load_from_path

Load settings from a specific path.

## Signature

```rust
pub fn load_from_path(path: P) -> Result<Self, SettingsError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Load settings from a specific path.

## Source
Lines 103–107 in `crates/oxide-settings/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-settings/src/lib.md) |
