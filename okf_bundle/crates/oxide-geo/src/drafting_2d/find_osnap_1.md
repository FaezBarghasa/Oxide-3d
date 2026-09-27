---
okf_version: "0.2"
type: Function
title: find_osnap
description: Find closest OSNAP candidate for cursor position.
resource: crates/oxide-geo/src/drafting_2d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:41:20Z"
concept_id: crates/oxide-geo/src/drafting_2d/find_osnap_1
language: rust
---

# find_osnap

Find closest OSNAP candidate for cursor position.

## Signature

```rust
pub fn find_osnap(&self, cursor: Point2D, tolerance: f64) -> Option<OsnapCandidate>
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Find closest OSNAP candidate for cursor position.
[must_use]

## Source
Lines 193–276 in `crates/oxide-geo/src/drafting_2d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drafting_2d](/crates/oxide-geo/src/drafting_2d.md) |
