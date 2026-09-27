---
okf_version: "0.2"
type: Class
title: OxdManifest
description: "Manifest stored in root of `.oxd` package."
resource: crates/oxide-persist/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-persist"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T11:33:51Z"
concept_id: crates/oxide-persist/src/lib/OxdManifest
language: rust
---

# OxdManifest

Manifest stored in root of `.oxd` package.

## Signature

```rust
pub struct OxdManifest
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Manifest stored in root of `.oxd` package.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `format_version`
- `document_id`
- `title`
- `created_at`

## Source
Lines 32–41 in `crates/oxide-persist/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-persist/src/lib.md) |
