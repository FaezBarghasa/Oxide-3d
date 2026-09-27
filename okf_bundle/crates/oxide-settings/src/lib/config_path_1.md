---
okf_version: "0.2"
type: Function
title: config_path
description: "Returns the standard path to the `settings.toml` configuration file."
resource: crates/oxide-settings/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-settings"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:34:16Z"
concept_id: crates/oxide-settings/src/lib/config_path_1
language: rust
---

# config_path

Returns the standard path to the `settings.toml` configuration file.

## Signature

```rust
pub fn config_path() -> Result<PathBuf, SettingsError>
```

## Visibility

- `pub`

## Docstring

Returns the standard path to the `settings.toml` configuration file.

## Source
Lines 84–89 in `crates/oxide-settings/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-settings/src/lib.md) |
