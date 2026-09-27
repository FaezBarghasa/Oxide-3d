---
okf_version: "0.2"
type: Function
title: resolve_alias
description: "Resolve command aliases (AutoCAD & OpenCADStudio standard)."
resource: crates/oxide-ui/src/command_prompt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:39:38Z"
concept_id: crates/oxide-ui/src/command_prompt/resolve_alias_1
language: rust
---

# resolve_alias

Resolve command aliases (AutoCAD & OpenCADStudio standard).

## Signature

```rust
pub fn resolve_alias(input: &str) -> &'static str
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Resolve command aliases (AutoCAD & OpenCADStudio standard).
[must_use]

## Source
Lines 44–84 in `crates/oxide-ui/src/command_prompt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_prompt](/crates/oxide-ui/src/command_prompt.md) |
