---
okf_version: "0.2"
type: Class
title: BufferId
description: Handle to an allocated device memory buffer.
resource: crates/oxide-compute/src/traits.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/traits/BufferId
language: rust
---

# BufferId

Handle to an allocated device memory buffer.

## Signature

```rust
pub struct BufferId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Handle to an allocated device memory buffer.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Source
Lines 110–110 in `crates/oxide-compute/src/traits.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [traits](/crates/oxide-compute/src/traits.md) |
| called_by | [allocate_buffer](/crates/oxide-compute/src/cpu/allocate_buffer.md) |
| called_by | [allocate_buffer](/crates/oxide-compute/src/native/allocate_buffer.md) |
