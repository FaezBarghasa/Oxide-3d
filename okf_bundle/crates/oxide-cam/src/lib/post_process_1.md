---
okf_version: "0.2"
type: Function
title: post_process
description: Format a list of toolpath points into a CNC G-code program string.
resource: crates/oxide-cam/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cam"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:51:21Z"
concept_id: crates/oxide-cam/src/lib/post_process_1
language: rust
---

# post_process

Format a list of toolpath points into a CNC G-code program string.

## Signature

```rust
pub fn post_process(
        dialect: PostProcessorDialect,
        program_name: &str,
        spindle_rpm: f64,
        points: &[ToolpathPoint],
    ) -> String
```

## Visibility

- `pub`

## Docstring

Format a list of toolpath points into a CNC G-code program string.

## Source
Lines 186–238 in `crates/oxide-cam/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-cam/src/lib.md) |
