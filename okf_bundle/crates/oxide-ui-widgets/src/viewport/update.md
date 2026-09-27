---
okf_version: "0.2"
type: Function
title: update
resource: crates/oxide-ui-widgets/src/viewport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui-widgets"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-ui-widgets/src/viewport/update
language: rust
---

# update

## Signature

```rust
impl ViewportWidget<'a> { fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<ViewportMessage>> }
```

## Type Parameters

- `'a`

## Source
Lines 72–129 in `crates/oxide-ui-widgets/src/viewport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [viewport](/crates/oxide-ui-widgets/src/viewport.md) |
