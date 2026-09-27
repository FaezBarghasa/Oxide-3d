---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-ui-widgets/src/viewport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui-widgets"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-ui-widgets/src/viewport/draw
language: rust
---

# draw

## Signature

```rust
impl ViewportWidget<'a> { fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> }
```

## Type Parameters

- `'a`

## Source
Lines 131–265 in `crates/oxide-ui-widgets/src/viewport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [viewport](/crates/oxide-ui-widgets/src/viewport.md) |
