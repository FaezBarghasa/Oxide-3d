---
okf_version: "0.2"
type: Function
title: render_cad_workspace
description: Render the SolidWorks-style Parametric CAD workspace.
resource: crates/oxide-ui/src/views/cad_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:41:20Z"
concept_id: crates/oxide-ui/src/views/cad_view/render_cad_workspace
language: rust
---

# render_cad_workspace

Render the SolidWorks-style Parametric CAD workspace.

## Signature

```rust
pub fn render_cad_workspace(app: &OxideApp) -> Element<'_, OxideUiMessage>
```

## Visibility

- `pub`

## Docstring

Render the SolidWorks-style Parametric CAD workspace.

## Source
Lines 10–84 in `crates/oxide-ui/src/views/cad_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cad_view](/crates/oxide-ui/src/views/cad_view.md) |
| calls | [viewport_canvas](/crates/oxide-ui-widgets/src/viewport/viewport_canvas.md) |
