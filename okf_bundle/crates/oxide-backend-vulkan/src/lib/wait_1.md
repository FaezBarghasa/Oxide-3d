---
okf_version: "0.2"
type: Function
title: wait
resource: crates/oxide-backend-vulkan/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-vulkan"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:05:28Z"
concept_id: crates/oxide-backend-vulkan/src/lib/wait_1
language: rust
---

# wait

## Signature

```rust
fn wait(&self, _event: &KernelEvent) -> Result<(), ComputeError>
```

## Source
Lines 81–83 in `crates/oxide-backend-vulkan/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-vulkan/src/lib.md) |
