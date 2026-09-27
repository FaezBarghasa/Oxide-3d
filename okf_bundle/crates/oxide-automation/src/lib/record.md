---
okf_version: "0.2"
type: Function
title: record
description: Record a dispatched command.
resource: crates/oxide-automation/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-automation"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-automation/src/lib/record
language: rust
---

# record

Record a dispatched command.

## Signature

```rust
impl MacroRecorder { pub fn record(&mut self, cmd: OxideCommand) }
```

## Visibility

- `pub`

## Docstring

Record a dispatched command.

## Source
Lines 39–43 in `crates/oxide-automation/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-automation/src/lib.md) |
