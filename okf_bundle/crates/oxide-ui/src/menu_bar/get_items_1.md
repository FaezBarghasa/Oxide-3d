---
okf_version: "0.2"
type: Function
title: get_items
description: Return items for a given menu category.
resource: crates/oxide-ui/src/menu_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:19:25Z"
concept_id: crates/oxide-ui/src/menu_bar/get_items_1
language: rust
---

# get_items

Return items for a given menu category.

## Signature

```rust
pub fn get_items(&self, category: MenuCategory) -> Vec<MenuItemDef>
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Return items for a given menu category.
[must_use]

## Source
Lines 68–202 in `crates/oxide-ui/src/menu_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-ui/src/menu_bar.md) |
