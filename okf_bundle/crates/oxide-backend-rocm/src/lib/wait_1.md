---
okf_version: "0.2"
type: Function
title: wait
resource: crates/oxide-backend-rocm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-rocm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:05:07Z"
concept_id: crates/oxide-backend-rocm/src/lib/wait_1
language: rust
---

# wait

## Signature

```rust
fn wait(&self, _event: &KernelEvent) -> Result<(), ComputeError>
```

## Source
Lines 83–85 in `crates/oxide-backend-rocm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-rocm/src/lib.md) |
