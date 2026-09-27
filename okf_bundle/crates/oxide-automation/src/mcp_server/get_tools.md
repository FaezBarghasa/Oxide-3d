---
okf_version: "0.2"
type: Function
title: get_tools
description: Return full list of available MCP tools for CAD automation.
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/get_tools
language: rust
---

# get_tools

Return full list of available MCP tools for CAD automation.

## Signature

```rust
impl McpServer { pub fn get_tools() -> Vec<McpToolDefinition> }
```

## Visibility

- `pub`

## Docstring

Return full list of available MCP tools for CAD automation.
[must_use]

## Source
Lines 152–200 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
