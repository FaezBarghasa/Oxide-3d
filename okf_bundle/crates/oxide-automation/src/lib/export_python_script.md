---
okf_version: "0.2"
type: Function
title: export_python_script
description: Export recorded commands as an executable Python automation script.
resource: crates/oxide-automation/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/lib/export_python_script
language: rust
---

# export_python_script

Export recorded commands as an executable Python automation script.

## Signature

```rust
impl MacroRecorder { pub fn export_python_script(&self) -> String }
```

## Visibility

- `pub`

## Docstring

Export recorded commands as an executable Python automation script.

## Source
Lines 46–91 in `crates/oxide-automation/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-automation/src/lib.md) |
