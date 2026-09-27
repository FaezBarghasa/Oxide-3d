---
okf_version: "0.2"
type: Function
title: eval_expression
description: Evaluates a parametric mathematical expression string with given variable scope.
resource: crates/oxide-script-rhai/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-script-rhai"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T22:04:56Z"
concept_id: crates/oxide-script-rhai/src/lib/eval_expression
language: rust
---

# eval_expression

Evaluates a parametric mathematical expression string with given variable scope.

## Signature

```rust
pub fn eval_expression(expr: &str, variables: &[(&str, f64)]) -> Result<f64, RhaiError>
```

## Visibility

- `pub`

## Docstring

Evaluates a parametric mathematical expression string with given variable scope.

## Source
Lines 15–36 in `crates/oxide-script-rhai/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-script-rhai/src/lib.md) |
| called_by | [test_eval_parametric_dimensions](/crates/oxide-script-rhai/src/lib/test_eval_parametric_dimensions.md) |
