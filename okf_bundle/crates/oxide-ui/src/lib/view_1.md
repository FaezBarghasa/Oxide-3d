---
okf_version: "0.2"
type: Function
title: view
description: Application view construction.
resource: crates/oxide-ui/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:41:20Z"
concept_id: crates/oxide-ui/src/lib/view_1
language: rust
---

# view

Application view construction.

## Signature

```rust
pub fn view(&self) -> Element<'_, OxideUiMessage>
```

## Visibility

- `pub`

## Docstring

Application view construction.

## Source
Lines 433–548 in `crates/oxide-ui/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-ui/src/lib.md) |
| calls | [viewport_canvas](/crates/oxide-ui-widgets/src/viewport/viewport_canvas.md) |
