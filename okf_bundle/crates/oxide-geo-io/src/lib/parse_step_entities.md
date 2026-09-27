---
okf_version: "0.2"
type: Function
title: parse_step_entities
description: Parse ISO 10303-21 STEP physical file lines into entity records.
resource: crates/oxide-geo-io/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:45:43Z"
concept_id: crates/oxide-geo-io/src/lib/parse_step_entities
language: rust
---

# parse_step_entities

Parse ISO 10303-21 STEP physical file lines into entity records.

## Signature

```rust
pub fn parse_step_entities(content: &str) -> Vec<StepEntity>
```

## Visibility

- `pub`

## Docstring

Parse ISO 10303-21 STEP physical file lines into entity records.

## Source
Lines 195–216 in `crates/oxide-geo-io/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-geo-io/src/lib.md) |
| called_by | [test_step_entity_parsing](/crates/oxide-geo-io/src/lib/test_step_entity_parsing.md) |
