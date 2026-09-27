---
okf_version: "0.2"
type: Class
title: PocketOperation
description: CAM Operation parameters.
resource: crates/oxide-cam/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cam"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:51:21Z"
concept_id: crates/oxide-cam/src/lib/PocketOperation
language: rust
---

# PocketOperation

CAM Operation parameters.

## Signature

```rust
pub struct PocketOperation
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

CAM Operation parameters.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `tool`
- `clearance_z`
- `retract_z`
- `stock_top_z`
- `target_depth_z`
- `stepdown_mm`
- `stepover_mm`
- `feedrate_mm_min`
- `plunge_feedrate_mm_min`
- `spindle_rpm`

## Source
Lines 50–71 in `crates/oxide-cam/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-cam/src/lib.md) |
