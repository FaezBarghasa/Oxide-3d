---
okf_version: "0.2"
type: Function
title: new
description: Create new multileader annotation.
resource: crates/oxide-geo/src/annotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo/src/annotation/new
language: rust
---

# new

Create new multileader annotation.

## Signature

```rust
impl MultiLeader { pub fn new(arrow_target: Point2D, landing_point: Point2D, text: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Docstring

Create new multileader annotation.
[must_use]

## Source
Lines 136–144 in `crates/oxide-geo/src/annotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotation](/crates/oxide-geo/src/annotation.md) |
