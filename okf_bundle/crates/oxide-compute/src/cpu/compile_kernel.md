---
okf_version: "0.2"
type: Function
title: compile_kernel
resource: crates/oxide-compute/src/cpu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/cpu/compile_kernel
language: rust
---

# compile_kernel

## Signature

```rust
impl CpuComputeDevice { fn compile_kernel(&self, _name: &str, _source: &str) -> Result<KernelId, ComputeError> }
```

## Source
Lines 129–132 in `crates/oxide-compute/src/cpu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cpu](/crates/oxide-compute/src/cpu.md) |
| calls | [KernelId](/crates/oxide-compute/src/traits/KernelId.md) |
