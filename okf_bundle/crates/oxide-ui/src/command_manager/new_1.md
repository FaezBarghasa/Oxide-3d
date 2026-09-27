---
okf_version: "0.2"
type: Function
title: new
description: Create a new CommandToolDef.
resource: crates/oxide-ui/src/command_manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:38:27Z"
concept_id: crates/oxide-ui/src/command_manager/new_1
language: rust
---

# new

Create a new CommandToolDef.

## Signature

```rust
pub fn new(
        name: impl Into<String>,
        icon_id: &'static str,
        action_id: &'static str,
        tooltip: impl Into<String>,
    ) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create a new CommandToolDef.
[must_use]

## Source
Lines 86–98 in `crates/oxide-ui/src/command_manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_manager](/crates/oxide-ui/src/command_manager.md) |
