---
okf_version: "0.2"
type: Function
title: internal
description: Create standard InternalError error (-32603).
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/internal_1
language: rust
---

# internal

Create standard InternalError error (-32603).

## Signature

```rust
pub fn internal(msg: impl Into<String>) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create standard InternalError error (-32603).
[must_use]

## Source
Lines 77–83 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
