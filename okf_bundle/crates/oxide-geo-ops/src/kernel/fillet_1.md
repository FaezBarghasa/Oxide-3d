---
okf_version: "0.2"
type: Function
title: fillet
resource: crates/oxide-geo-ops/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/kernel/fillet_1
language: rust
---

# fillet

## Signature

```rust
fn fillet(
        &self,
        solid: SolidKey,
        _edges: &[EdgeKey],
        _opts: FilletOptions,
    ) -> KernelResult<SolidKey>
```

## Source
Lines 105–118 in `crates/oxide-geo-ops/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/oxide-geo-ops/src/kernel.md) |
