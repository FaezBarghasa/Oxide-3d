---
okf_version: "0.2"
type: Module
title: mcp_server
description: Native Model Context Protocol (MCP) Server Endpoint for Oxide-3D Automation.
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server
language: rust
---

# mcp_server

Native Model Context Protocol (MCP) Server Endpoint for Oxide-3D Automation.

## Docstring

Native Model Context Protocol (MCP) Server Endpoint for Oxide-3D Automation.

Enables external AI agents and automation tools to control Oxide-3D via JSON-RPC 2.0.
Supports session initialization, entity inspection, command execution, and viewport snapshot queries.

## Relationships

| Type | Target |
|------|--------|
| related | [McpRequest](/crates/oxide-automation/src/mcp_server/McpRequest.md) |
| related | [McpResponse](/crates/oxide-automation/src/mcp_server/McpResponse.md) |
| related | [McpError](/crates/oxide-automation/src/mcp_server/McpError.md) |
| related | [method_not_found](/crates/oxide-automation/src/mcp_server/method_not_found.md) |
| related | [invalid_params](/crates/oxide-automation/src/mcp_server/invalid_params.md) |
| related | [internal](/crates/oxide-automation/src/mcp_server/internal.md) |
| related | [method_not_found](/crates/oxide-automation/src/mcp_server/method_not_found.md) |
| related | [invalid_params](/crates/oxide-automation/src/mcp_server/invalid_params.md) |
| related | [internal](/crates/oxide-automation/src/mcp_server/internal.md) |
| related | [McpServerCapabilities](/crates/oxide-automation/src/mcp_server/McpServerCapabilities.md) |
| related | [McpToolDefinition](/crates/oxide-automation/src/mcp_server/McpToolDefinition.md) |
| related | [McpSessionState](/crates/oxide-automation/src/mcp_server/McpSessionState.md) |
| related | [McpServer](/crates/oxide-automation/src/mcp_server/McpServer.md) |
| related | [new](/crates/oxide-automation/src/mcp_server/new.md) |
| related | [get_tools](/crates/oxide-automation/src/mcp_server/get_tools.md) |
| related | [handle_request](/crates/oxide-automation/src/mcp_server/handle_request.md) |
| related | [new](/crates/oxide-automation/src/mcp_server/new.md) |
| related | [get_tools](/crates/oxide-automation/src/mcp_server/get_tools.md) |
| related | [handle_request](/crates/oxide-automation/src/mcp_server/handle_request.md) |
| related | [test_mcp_server_initialize](/crates/oxide-automation/src/mcp_server/test_mcp_server_initialize.md) |
| related | [test_mcp_tool_execution](/crates/oxide-automation/src/mcp_server/test_mcp_tool_execution.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
