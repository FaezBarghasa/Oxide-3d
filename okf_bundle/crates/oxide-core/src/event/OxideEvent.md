---
okf_version: "0.2"
type: Class
title: OxideEvent
description: "Domain events emitted during CAD modeling, simulation, and document lifecycle."
resource: crates/oxide-core/src/event.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-core/src/event/OxideEvent
language: rust
---

# OxideEvent

Domain events emitted during CAD modeling, simulation, and document lifecycle.

## Signature

```rust
pub enum OxideEvent
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Domain events emitted during CAD modeling, simulation, and document lifecycle.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `id`
- `description`
- `key`
- `selected`

## Source
Lines 6–29 in `crates/oxide-core/src/event.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [event](/crates/oxide-core/src/event.md) |
