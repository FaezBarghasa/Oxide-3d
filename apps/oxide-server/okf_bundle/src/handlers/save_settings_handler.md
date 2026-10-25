---
okf_version: "0.2"
type: Function
title: save_settings_handler
description: Save settings handler.
resource: src/handlers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src"
  - "domain:handlers.rs"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:39:07Z"
concept_id: src/handlers/save_settings_handler
language: rust
---

# save_settings_handler

Save settings handler.

## Signature

```rust
pub fn save_settings_handler(Json(settings): Json<OxideSettings>) -> impl IntoResponse
```

## Visibility

- `pub`

## Docstring

Save settings handler.

## Source
Lines 100–103 in `src/handlers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handlers](/src/handlers.md) |
