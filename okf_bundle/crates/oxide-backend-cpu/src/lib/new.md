---
okf_version: "0.2"
type: Function
title: new
description: Initialize CPU execution backend with host thread topology.
resource: crates/oxide-backend-cpu/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-cpu"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-backend-cpu/src/lib/new
language: rust
---

# new

Initialize CPU execution backend with host thread topology.

## Signature

```rust
impl CpuBackend { pub fn new() -> Self }
```

## Visibility

- `pub`

## Docstring

Initialize CPU execution backend with host thread topology.
[must_use]

## Source
Lines 36–59 in `crates/oxide-backend-cpu/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-cpu/src/lib.md) |
