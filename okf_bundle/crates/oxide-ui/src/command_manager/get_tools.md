---
okf_version: "0.2"
type: Function
title: get_tools
description: Return list of tools in the specified tab.
resource: crates/oxide-ui/src/command_manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:38:27Z"
concept_id: crates/oxide-ui/src/command_manager/get_tools
language: rust
---

# get_tools

Return list of tools in the specified tab.

## Signature

```rust
impl CommandManagerModel { pub fn get_tools(&self, tab: CommandTab) -> Vec<CommandToolDef> }
```

## Visibility

- `pub`

## Docstring

Return list of tools in the specified tab.
[must_use]

## Source
Lines 163–792 in `crates/oxide-ui/src/command_manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_manager](/crates/oxide-ui/src/command_manager.md) |
