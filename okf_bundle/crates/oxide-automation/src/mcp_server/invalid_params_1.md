---
okf_version: "0.2"
type: Function
title: invalid_params
description: Create standard InvalidParams error (-32602).
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/invalid_params_1
language: rust
---

# invalid_params

Create standard InvalidParams error (-32602).

## Signature

```rust
pub fn invalid_params(msg: impl Into<String>) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create standard InvalidParams error (-32602).
[must_use]

## Source
Lines 67–73 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
