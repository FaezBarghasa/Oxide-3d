---
okf_version: "0.2"
type: Function
title: create_primitive_handler
description: B-Rep Primitive generation handler.
resource: apps/oxide-server/src/handlers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:apps"
  - "domain:oxide-server"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:39:07Z"
concept_id: apps/oxide-server/src/handlers/create_primitive_handler
language: rust
---

# create_primitive_handler

B-Rep Primitive generation handler.

## Signature

```rust
pub fn create_primitive_handler(Json(req): Json<PrimitiveReq>) -> impl IntoResponse
```

## Visibility

- `pub`

## Docstring

B-Rep Primitive generation handler.

## Source
Lines 106–128 in `apps/oxide-server/src/handlers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handlers](/apps/oxide-server/src/handlers.md) |
