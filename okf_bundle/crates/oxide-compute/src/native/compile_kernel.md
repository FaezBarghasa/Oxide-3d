---
okf_version: "0.2"
type: Function
title: compile_kernel
resource: crates/oxide-compute/src/native.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/native/compile_kernel
language: rust
---

# compile_kernel

## Signature

```rust
impl NativeComputeDevice { fn compile_kernel(&self, name: &str, source: &str) -> Result<KernelId, ComputeError> }
```

## Source
Lines 149–157 in `crates/oxide-compute/src/native.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [native](/crates/oxide-compute/src/native.md) |
| calls | [KernelId](/crates/oxide-compute/src/traits/KernelId.md) |
