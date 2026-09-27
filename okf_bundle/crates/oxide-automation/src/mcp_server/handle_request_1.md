---
okf_version: "0.2"
type: Function
title: handle_request
description: Dispatch and execute an incoming JSON-RPC request.
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/handle_request_1
language: rust
---

# handle_request

Dispatch and execute an incoming JSON-RPC request.

## Signature

```rust
pub fn handle_request(&mut self, req: McpRequest) -> McpResponse
```

## Visibility

- `pub`

## Docstring

Dispatch and execute an incoming JSON-RPC request.

## Source
Lines 203–328 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
