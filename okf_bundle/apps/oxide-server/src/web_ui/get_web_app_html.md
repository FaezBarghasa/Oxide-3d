---
okf_version: "0.2"
type: Function
title: get_web_app_html
description: Returns the complete HTML/JS/CSS client application string.
resource: apps/oxide-server/src/web_ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:apps"
  - "domain:oxide-server"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-13T09:03:30Z"
concept_id: apps/oxide-server/src/web_ui/get_web_app_html
language: rust
---

# get_web_app_html

Returns the complete HTML/JS/CSS client application string.

## Signature

```rust
pub fn get_web_app_html() -> &'static str
```

## Visibility

- `pub`

## Docstring

Returns the complete HTML/JS/CSS client application string.

## Source
Lines 6–1600 in `apps/oxide-server/src/web_ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [web_ui](/apps/oxide-server/src/web_ui.md) |
| called_by | [main](/apps/oxide-server/src/main/main.md) |
