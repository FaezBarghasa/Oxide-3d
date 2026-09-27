---
okf_version: "0.2"
type: Function
title: extrude
resource: crates/oxide-geo-ops/src/kernel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/kernel/extrude_1
language: rust
---

# extrude

## Signature

```rust
fn extrude(
        &self,
        profile: FaceKey,
        direction: [f64; 3],
        opts: ExtrudeOptions,
    ) -> KernelResult<SolidKey>
```

## Source
Lines 90–103 in `crates/oxide-geo-ops/src/kernel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kernel](/crates/oxide-geo-ops/src/kernel.md) |
| calls | [extrude_face](/crates/oxide-geo-ops/src/feature_ops/extrude_face.md) |
