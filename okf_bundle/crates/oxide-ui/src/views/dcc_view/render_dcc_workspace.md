---
okf_version: "0.2"
type: Function
title: render_dcc_workspace
description: Render the complete DCC Workspace view.
resource: crates/oxide-ui/src/views/dcc_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:41:20Z"
concept_id: crates/oxide-ui/src/views/dcc_view/render_dcc_workspace
language: rust
---

# render_dcc_workspace

Render the complete DCC Workspace view.

## Signature

```rust
pub fn render_dcc_workspace(app: &OxideApp) -> Element<'_, OxideUiMessage>
```

## Visibility

- `pub`

## Docstring

Render the complete DCC Workspace view.

## Source
Lines 12–280 in `crates/oxide-ui/src/views/dcc_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dcc_view](/crates/oxide-ui/src/views/dcc_view.md) |
| calls | [viewport_canvas](/crates/oxide-ui-widgets/src/viewport/viewport_canvas.md) |
