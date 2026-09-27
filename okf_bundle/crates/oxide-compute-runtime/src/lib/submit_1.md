---
okf_version: "0.2"
type: Function
title: submit
description: Submit a compute job with async execution and cancellation safety.
resource: crates/oxide-compute-runtime/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute-runtime"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute-runtime/src/lib/submit_1
language: rust
---

# submit

Submit a compute job with async execution and cancellation safety.

## Signature

```rust
pub fn submit(
        &self,
        cancel: CancellationToken,
        job: ComputeJob,
        args: &[KernelArg],
    ) -> Result<(), ComputeError>
```

## Visibility

- `pub`

## Docstring

Submit a compute job with async execution and cancellation safety.

## Source
Lines 136–153 in `crates/oxide-compute-runtime/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-compute-runtime/src/lib.md) |
