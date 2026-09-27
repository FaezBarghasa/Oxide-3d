---
okf_version: "0.2"
type: Class
title: EngineeringChangeOrder
description: Engineering Change Order (ECO) tracking.
resource: crates/oxide-plm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-plm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-plm/src/lib/EngineeringChangeOrder
language: rust
---

# EngineeringChangeOrder

Engineering Change Order (ECO) tracking.

## Signature

```rust
pub struct EngineeringChangeOrder
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Engineering Change Order (ECO) tracking.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `eco_number`
- `description`
- `affected_items`
- `approved`

## Source
Lines 82–91 in `crates/oxide-plm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-plm/src/lib.md) |
