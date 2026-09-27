---
okf_version: "0.2"
type: Function
title: tangent
description: "Evaluate tangent derivative vector `C'(t)` on curve at parameter `t`."
resource: crates/oxide-geo/src/curve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo/src/curve/tangent
language: rust
---

# tangent

Evaluate tangent derivative vector `C'(t)` on curve at parameter `t`.

## Signature

```rust
impl Curve3d { pub fn tangent(&self, t: f64) -> [f64; 3] }
```

## Visibility

- `pub`

## Docstring

Evaluate tangent derivative vector `C'(t)` on curve at parameter `t`.
[must_use]

## Source
Lines 74–87 in `crates/oxide-geo/src/curve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [curve](/crates/oxide-geo/src/curve.md) |
