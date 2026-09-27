---
okf_version: "0.2"
type: Class
title: FeatureTreeNode
description: Single entry node in the FeatureManager Tree.
resource: crates/oxide-ui/src/feature_manager.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:19:25Z"
concept_id: crates/oxide-ui/src/feature_manager/FeatureTreeNode
language: rust
---

# FeatureTreeNode

Single entry node in the FeatureManager Tree.

## Signature

```rust
pub struct FeatureTreeNode
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Single entry node in the FeatureManager Tree.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `label`
- `kind`
- `parent_id`
- `children`
- `is_selected`
- `is_expanded`

## Source
Lines 66–81 in `crates/oxide-ui/src/feature_manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [feature_manager](/crates/oxide-ui/src/feature_manager.md) |
