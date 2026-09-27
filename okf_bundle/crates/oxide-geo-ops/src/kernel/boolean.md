---
okf_version: "0.2"
type: Function
title: boolean
resource: crates/oxide-geo-ops/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/kernel/boolean
language: rust
---

# boolean

## Signature

```rust
impl NativeGeometryKernel { fn boolean(&self, a: SolidKey, b: SolidKey, opts: BooleanOptions) -> KernelResult<SolidKey> }
```

## Source
Lines 82–88 in `crates/oxide-geo-ops/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/oxide-geo-ops/src/kernel.md) |
| calls | [boolean_op](/crates/oxide-geo-ops/src/boolean/boolean_op.md) |
