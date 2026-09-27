---
okf_version: "0.2"
type: Function
title: viewport_canvas
description: Convenience helper to create a canvas element backed by the ViewportWidget.
resource: crates/oxide-ui-widgets/src/viewport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui-widgets"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-ui-widgets/src/viewport/viewport_canvas
language: rust
---

# viewport_canvas

Convenience helper to create a canvas element backed by the ViewportWidget.

## Signature

```rust
pub fn viewport_canvas(
    camera: &'a Camera,
    mesh: &'a TriMesh,
    map_fn: impl Fn(ViewportMessage) -> Message + 'a,
) -> Element<'a, Message>
```

## Type Parameters

- `'a`
- `Message: 'a`

## Visibility

- `pub`

## Docstring

Convenience helper to create a canvas element backed by the ViewportWidget.

## Source
Lines 269–280 in `crates/oxide-ui-widgets/src/viewport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [viewport](/crates/oxide-ui-widgets/src/viewport.md) |
| called_by | [view](/crates/oxide-ui/src/lib/view.md) |
| called_by | [render_cad_workspace](/crates/oxide-ui/src/views/cad_view/render_cad_workspace.md) |
| called_by | [render_dcc_workspace](/crates/oxide-ui/src/views/dcc_view/render_dcc_workspace.md) |
| called_by | [render_drafting_workspace](/crates/oxide-ui/src/views/drafting_view/render_drafting_workspace.md) |
