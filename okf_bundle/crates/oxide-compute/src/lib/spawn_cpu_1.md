---
okf_version: "0.2"
type: Function
title: spawn_cpu
description: Submit a task to the Rayon CPU thread pool.
resource: crates/oxide-compute/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T22:15:03Z"
concept_id: crates/oxide-compute/src/lib/spawn_cpu_1
language: rust
---

# spawn_cpu

Submit a task to the Rayon CPU thread pool.

## Signature

```rust
pub fn spawn_cpu(&self, f: F) -> tokio::sync::oneshot::Receiver<R>
```

## Type Parameters

- `F`
- `R`

## Visibility

- `pub`

## Docstring

Submit a task to the Rayon CPU thread pool.

## Source
Lines 47–58 in `crates/oxide-compute/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-compute/src/lib.md) |
