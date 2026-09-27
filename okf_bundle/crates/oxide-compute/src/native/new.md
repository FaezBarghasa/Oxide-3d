---
okf_version: "0.2"
type: Function
title: new
description: Create a new native device representation for a given backend kind.
resource: crates/oxide-compute/src/native.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/native/new
language: rust
---

# new

Create a new native device representation for a given backend kind.

## Signature

```rust
impl NativeComputeDevice { pub fn new(backend: BackendKind, name: &str, total_mem: u64, is_unified: bool) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new native device representation for a given backend kind.
[must_use]

## Source
Lines 30–52 in `crates/oxide-compute/src/native.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [native](/crates/oxide-compute/src/native.md) |
