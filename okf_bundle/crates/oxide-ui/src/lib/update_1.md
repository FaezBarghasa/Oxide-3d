---
okf_version: "0.2"
type: Function
title: update
description: Application update logic.
resource: crates/oxide-ui/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:41:20Z"
concept_id: crates/oxide-ui/src/lib/update_1
language: rust
---

# update

Application update logic.

## Signature

```rust
pub fn update(&mut self, message: OxideUiMessage) -> Task<OxideUiMessage>
```

## Visibility

- `pub`

## Docstring

Application update logic.

## Source
Lines 252–430 in `crates/oxide-ui/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-ui/src/lib.md) |
