---
okf_version: "0.2"
type: Function
title: catch_ffi_boundary
description: Safely execute an FFI block while catching potential unhandled foreign panics or aborts.
resource: crates/oxide-ffi-safe/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ffi-safe"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-ffi-safe/src/lib/catch_ffi_boundary
language: rust
---

# catch_ffi_boundary

Safely execute an FFI block while catching potential unhandled foreign panics or aborts.

## Signature

```rust
pub fn catch_ffi_boundary(f: F) -> Result<R, ComputeError>
```

## Type Parameters

- `F`
- `R`

## Visibility

- `pub`

## Docstring

Safely execute an FFI block while catching potential unhandled foreign panics or aborts.

## Source
Lines 7–17 in `crates/oxide-ffi-safe/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-ffi-safe/src/lib.md) |
