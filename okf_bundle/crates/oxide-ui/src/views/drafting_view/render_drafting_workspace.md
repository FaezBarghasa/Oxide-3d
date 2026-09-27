---
okf_version: "0.2"
type: Function
title: render_drafting_workspace
description: Render the OpenCADStudio 2D drafting workspace.
resource: crates/oxide-ui/src/views/drafting_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:41:20Z"
concept_id: crates/oxide-ui/src/views/drafting_view/render_drafting_workspace
language: rust
---

# render_drafting_workspace

Render the OpenCADStudio 2D drafting workspace.

## Signature

```rust
pub fn render_drafting_workspace(app: &OxideApp) -> Element<'_, OxideUiMessage>
```

## Visibility

- `pub`

## Docstring

Render the OpenCADStudio 2D drafting workspace.

## Source
Lines 10–106 in `crates/oxide-ui/src/views/drafting_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drafting_view](/crates/oxide-ui/src/views/drafting_view.md) |
| calls | [viewport_canvas](/crates/oxide-ui-widgets/src/viewport/viewport_canvas.md) |
