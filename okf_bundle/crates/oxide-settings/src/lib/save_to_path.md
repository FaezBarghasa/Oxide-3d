---
okf_version: "0.2"
type: Function
title: save_to_path
description: Save current settings to a specific path.
resource: crates/oxide-settings/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-settings"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:34:16Z"
concept_id: crates/oxide-settings/src/lib/save_to_path
language: rust
---

# save_to_path

Save current settings to a specific path.

## Signature

```rust
impl OxideSettings { pub fn save_to_path(&self, path: P) -> Result<(), SettingsError> }
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Save current settings to a specific path.

## Source
Lines 116–123 in `crates/oxide-settings/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-settings/src/lib.md) |
