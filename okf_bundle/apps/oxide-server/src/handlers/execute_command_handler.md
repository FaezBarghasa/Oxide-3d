---
okf_version: "0.2"
type: Function
title: execute_command_handler
description: CAD Command execution interpreter handler.
resource: apps/oxide-server/src/handlers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:apps"
  - "domain:oxide-server"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:39:07Z"
concept_id: apps/oxide-server/src/handlers/execute_command_handler
language: rust
---

# execute_command_handler

CAD Command execution interpreter handler.

## Signature

```rust
pub fn execute_command_handler(Json(req): Json<CommandReq>) -> impl IntoResponse
```

## Visibility

- `pub`

## Docstring

CAD Command execution interpreter handler.

## Source
Lines 246–270 in `apps/oxide-server/src/handlers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handlers](/apps/oxide-server/src/handlers.md) |
