---
okf_version: "0.2"
type: Class
title: McpError
description: JSON-RPC 2.0 Error Object.
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/McpError
language: rust
---

# McpError

JSON-RPC 2.0 Error Object.

## Signature

```rust
pub struct McpError
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

JSON-RPC 2.0 Error Object.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `code`
- `message`
- `data`

## Source
Lines 44–52 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
