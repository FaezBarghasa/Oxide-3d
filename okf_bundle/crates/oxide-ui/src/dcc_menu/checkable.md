---
okf_version: "0.2"
type: Function
title: checkable
description: Create a toggleable / checkable item.
resource: crates/oxide-ui/src/dcc_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:27:52Z"
concept_id: crates/oxide-ui/src/dcc_menu/checkable
language: rust
---

# checkable

Create a toggleable / checkable item.

## Signature

```rust
impl DccMenuItemDef { pub fn checkable(
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<&str>,
        checked: bool,
    ) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a toggleable / checkable item.
[must_use]

## Source
Lines 105–120 in `crates/oxide-ui/src/dcc_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dcc_menu](/crates/oxide-ui/src/dcc_menu.md) |
