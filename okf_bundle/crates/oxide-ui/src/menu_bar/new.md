---
okf_version: "0.2"
type: Function
title: new
description: Create a new menu item.
resource: crates/oxide-ui/src/menu_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:19:25Z"
concept_id: crates/oxide-ui/src/menu_bar/new
language: rust
---

# new

Create a new menu item.

## Signature

```rust
impl MenuItemDef { pub fn new(label: impl Into<String>, action_id: &'static str, shortcut: Option<&str>) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new menu item.
[must_use]

## Source
Lines 40–47 in `crates/oxide-ui/src/menu_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-ui/src/menu_bar.md) |
