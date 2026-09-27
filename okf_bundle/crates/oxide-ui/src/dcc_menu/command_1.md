---
okf_version: "0.2"
type: Function
title: command
description: Create a standard command item.
resource: crates/oxide-ui/src/dcc_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:27:52Z"
concept_id: crates/oxide-ui/src/dcc_menu/command_1
language: rust
---

# command

Create a standard command item.

## Signature

```rust
pub fn command(
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<&str>,
    ) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create a standard command item.
[must_use]

## Source
Lines 87–101 in `crates/oxide-ui/src/dcc_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dcc_menu](/crates/oxide-ui/src/dcc_menu.md) |
