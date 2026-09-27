---
okf_version: "0.2"
type: Function
title: select
description: Select optimal kernel for a given category and target hardware backend.
resource: crates/oxide-kernels/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:05:26Z"
concept_id: crates/oxide-kernels/src/lib/select
language: rust
---

# select

Select optimal kernel for a given category and target hardware backend.

## Signature

```rust
impl KernelRegistry { pub fn select(&self, category: KernelCategory, backend: BackendKind) -> KernelHandle }
```

## Visibility

- `pub`

## Docstring

Select optimal kernel for a given category and target hardware backend.
[must_use]

## Source
Lines 44–61 in `crates/oxide-kernels/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-kernels/src/lib.md) |
