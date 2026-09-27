---
okf_version: "0.2"
type: Function
title: run_script_source
description: Helper to execute a Python macro or batch script.
resource: crates/oxide-script-python/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-script-python"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T20:53:34Z"
concept_id: crates/oxide-script-python/src/lib/run_script_source
language: rust
---

# run_script_source

Helper to execute a Python macro or batch script.

## Signature

```rust
pub fn run_script_source(source: &str) -> Result<(), PythonScriptError>
```

## Visibility

- `pub`

## Docstring

Helper to execute a Python macro or batch script.

## Source
Lines 14–17 in `crates/oxide-script-python/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-script-python/src/lib.md) |
