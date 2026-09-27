---
okf_version: "0.2"
type: Class
title: McpRequest
description: JSON-RPC 2.0 Request Object.
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/McpRequest
language: rust
---

# McpRequest

JSON-RPC 2.0 Request Object.

## Signature

```rust
pub struct McpRequest
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

JSON-RPC 2.0 Request Object.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `jsonrpc`
- `id`
- `method`
- `params`

## Source
Lines 15–25 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
