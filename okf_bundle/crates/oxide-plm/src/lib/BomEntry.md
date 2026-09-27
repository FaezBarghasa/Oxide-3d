---
okf_version: "0.2"
type: Class
title: BomEntry
description: Bill of Materials (BOM) parent-child entry.
resource: crates/oxide-plm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-plm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-plm/src/lib/BomEntry
language: rust
---

# BomEntry

Bill of Materials (BOM) parent-child entry.

## Signature

```rust
pub struct BomEntry
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Bill of Materials (BOM) parent-child entry.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `parent`
- `child`
- `quantity`
- `find_number`

## Source
Lines 58–67 in `crates/oxide-plm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-plm/src/lib.md) |
