---
okf_version: "0.2"
type: Function
title: new
description: Initialize a new WASM plugin host engine.
resource: crates/oxide-plugins-wasm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-plugins-wasm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T20:53:53Z"
concept_id: crates/oxide-plugins-wasm/src/lib/new
language: rust
---

# new

Initialize a new WASM plugin host engine.

## Signature

```rust
impl WasmPluginHost { pub fn new() -> Result<Self, PluginError> }
```

## Visibility

- `pub`

## Docstring

Initialize a new WASM plugin host engine.

## Source
Lines 38–41 in `crates/oxide-plugins-wasm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-plugins-wasm/src/lib.md) |
