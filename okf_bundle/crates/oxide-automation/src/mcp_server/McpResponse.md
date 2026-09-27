---
okf_version: "0.2"
type: Class
title: McpResponse
description: JSON-RPC 2.0 Response Object.
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/McpResponse
language: rust
---

# McpResponse

JSON-RPC 2.0 Response Object.

## Signature

```rust
pub struct McpResponse
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

JSON-RPC 2.0 Response Object.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `jsonrpc`
- `id`
- `result`
- `error`

## Source
Lines 29–40 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
