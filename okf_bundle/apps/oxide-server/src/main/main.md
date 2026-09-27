---
okf_version: "0.2"
type: Function
title: main
description: "[tokio::main]"
resource: apps/oxide-server/src/main.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:apps"
  - "domain:oxide-server"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:39:17Z"
concept_id: apps/oxide-server/src/main/main
language: rust
---

# main

[tokio::main]

## Signature

```rust
fn main() -> Result<(), Box<dyn std::error::Error>>
```

## Decorators

- `tokio::main`

## Docstring

[tokio::main]

## Source
Lines 13–40 in `apps/oxide-server/src/main.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [src](/apps/oxide-server/src/main.md) |
| calls | [init](/crates/oxide-telemetry/src/lib/init.md) |
| calls | [get_web_app_html](/apps/oxide-server/src/web_ui/get_web_app_html.md) |
