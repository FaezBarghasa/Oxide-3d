---
okf_version: "0.2"
type: Class
title: McpSessionState
description: State of an active Oxide-3D automation session.
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/McpSessionState
language: rust
---

# McpSessionState

State of an active Oxide-3D automation session.

## Signature

```rust
pub struct McpSessionState
```

## Decorators

- `derive(Debug, Default, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

State of an active Oxide-3D automation session.
[derive(Debug, Default, Clone, Serialize, Deserialize)]

## Methods

- `session_id`
- `active_document`
- `active_layer`
- `entity_count`
- `camera_eye`
- `variables`

## Source
Lines 110–123 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
