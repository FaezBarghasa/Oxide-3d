---
okf_version: "0.2"
type: Function
title: load_or_default
description: "Load settings from disk or return default if file doesn't exist."
resource: crates/oxide-settings/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-settings"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:34:16Z"
concept_id: crates/oxide-settings/src/lib/load_or_default
language: rust
---

# load_or_default

Load settings from disk or return default if file doesn't exist.

## Signature

```rust
impl OxideSettings { pub fn load_or_default() -> Self }
```

## Visibility

- `pub`

## Docstring

Load settings from disk or return default if file doesn't exist.

## Source
Lines 92–94 in `crates/oxide-settings/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-settings/src/lib.md) |
