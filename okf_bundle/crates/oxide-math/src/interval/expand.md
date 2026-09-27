---
okf_version: "0.2"
type: Function
title: expand
description: Expand interval by a margin.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/expand
language: rust
---

# expand

Expand interval by a margin.

## Signature

```rust
impl Interval { pub fn expand(&self, margin: f64) -> Self }
```

## Visibility

- `pub`

## Docstring

Expand interval by a margin.
[must_use]

## Source
Lines 79–84 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
