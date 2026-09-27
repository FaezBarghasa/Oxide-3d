---
okf_version: "0.2"
type: Function
title: method_not_found
description: Create standard MethodNotFound error (-32601).
resource: crates/oxide-automation/src/mcp_server.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/mcp_server/method_not_found
language: rust
---

# method_not_found

Create standard MethodNotFound error (-32601).

## Signature

```rust
impl McpError { pub fn method_not_found(method: &str) -> Self }
```

## Visibility

- `pub`

## Docstring

Create standard MethodNotFound error (-32601).
[must_use]

## Source
Lines 57–63 in `crates/oxide-automation/src/mcp_server.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp_server](/crates/oxide-automation/src/mcp_server.md) |
