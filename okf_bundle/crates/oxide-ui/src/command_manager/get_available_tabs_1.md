---
okf_version: "0.2"
type: Function
title: get_available_tabs
description: Return available tabs for current document context.
resource: crates/oxide-ui/src/command_manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:38:27Z"
concept_id: crates/oxide-ui/src/command_manager/get_available_tabs_1
language: rust
---

# get_available_tabs

Return available tabs for current document context.

## Signature

```rust
pub fn get_available_tabs(&self) -> Vec<CommandTab>
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Return available tabs for current document context.
[must_use]

## Source
Lines 131–159 in `crates/oxide-ui/src/command_manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_manager](/crates/oxide-ui/src/command_manager.md) |
