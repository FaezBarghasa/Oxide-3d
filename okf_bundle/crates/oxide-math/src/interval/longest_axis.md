---
okf_version: "0.2"
type: Function
title: longest_axis
description: "Get the longest axis (0=x, 1=y, 2=z)."
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/longest_axis
language: rust
---

# longest_axis

Get the longest axis (0=x, 1=y, 2=z).

## Signature

```rust
impl Interval3d { pub fn longest_axis(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Get the longest axis (0=x, 1=y, 2=z).
[must_use]

## Source
Lines 186–197 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
