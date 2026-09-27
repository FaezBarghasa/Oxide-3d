---
okf_version: "0.2"
type: Function
title: get_menu_items
description: Retrieve the full menu tree for a given category.
resource: crates/oxide-ui/src/dcc_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:27:52Z"
concept_id: crates/oxide-ui/src/dcc_menu/get_menu_items_1
language: rust
---

# get_menu_items

Retrieve the full menu tree for a given category.

## Signature

```rust
pub fn get_menu_items(&self, cat: DccMenuCategory) -> Vec<DccMenuItemDef>
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Retrieve the full menu tree for a given category.
[must_use]

## Source
Lines 188–204 in `crates/oxide-ui/src/dcc_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dcc_menu](/crates/oxide-ui/src/dcc_menu.md) |
