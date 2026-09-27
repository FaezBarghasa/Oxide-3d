---
okf_version: "0.2"
type: Function
title: submenu
description: Create a submenu with children items.
resource: crates/oxide-ui/src/dcc_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:27:52Z"
concept_id: crates/oxide-ui/src/dcc_menu/submenu_1
language: rust
---

# submenu

Create a submenu with children items.

## Signature

```rust
pub fn submenu(label: impl Into<String>, children: Vec<Self>) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create a submenu with children items.
[must_use]

## Source
Lines 124–135 in `crates/oxide-ui/src/dcc_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dcc_menu](/crates/oxide-ui/src/dcc_menu.md) |
