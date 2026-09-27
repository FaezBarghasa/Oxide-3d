---
okf_version: "0.2"
type: Module
title: dfm
description: Design for Manufacturability (DFM) Analysis Engine.
resource: crates/oxide-geo-ops/src/dfm.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:43:01Z"
concept_id: crates/oxide-geo-ops/src/dfm
language: rust
---

# dfm

Design for Manufacturability (DFM) Analysis Engine.

## Docstring

Design for Manufacturability (DFM) Analysis Engine.

Covers:
- Additive Manufacturing (3D Printing): Overhang angles, support volume, wall thickness.
- Subtractive (CNC Milling Aluminum): Tool accessibility cones, internal corner fillet radius, pocket aspect ratio.
- Injection Molded Plastics: Draft angles against mold pull direction, parting line analysis, wall thickness variation.

## Relationships

| Type | Target |
|------|--------|
| related | [DfmSeverity](/crates/oxide-geo-ops/src/dfm/DfmSeverity.md) |
| related | [DfmIssue](/crates/oxide-geo-ops/src/dfm/DfmIssue.md) |
| related | [AdditiveDfmConfig](/crates/oxide-geo-ops/src/dfm/AdditiveDfmConfig.md) |
| related | [default](/crates/oxide-geo-ops/src/dfm/default.md) |
| related | [default](/crates/oxide-geo-ops/src/dfm/default.md) |
| related | [CncMillingDfmConfig](/crates/oxide-geo-ops/src/dfm/CncMillingDfmConfig.md) |
| related | [default](/crates/oxide-geo-ops/src/dfm/default.md) |
| related | [default](/crates/oxide-geo-ops/src/dfm/default.md) |
| related | [InjectionMoldingDfmConfig](/crates/oxide-geo-ops/src/dfm/InjectionMoldingDfmConfig.md) |
| related | [default](/crates/oxide-geo-ops/src/dfm/default.md) |
| related | [default](/crates/oxide-geo-ops/src/dfm/default.md) |
| related | [DfmEngine](/crates/oxide-geo-ops/src/dfm/DfmEngine.md) |
| related | [analyze_additive](/crates/oxide-geo-ops/src/dfm/analyze_additive.md) |
| related | [analyze_cnc_milling](/crates/oxide-geo-ops/src/dfm/analyze_cnc_milling.md) |
| related | [analyze_injection_molding](/crates/oxide-geo-ops/src/dfm/analyze_injection_molding.md) |
| related | [analyze_additive](/crates/oxide-geo-ops/src/dfm/analyze_additive.md) |
| related | [analyze_cnc_milling](/crates/oxide-geo-ops/src/dfm/analyze_cnc_milling.md) |
| related | [analyze_injection_molding](/crates/oxide-geo-ops/src/dfm/analyze_injection_molding.md) |
| related | [create_test_cube](/crates/oxide-geo-ops/src/dfm/create_test_cube.md) |
| related | [test_dfm_additive_overhang](/crates/oxide-geo-ops/src/dfm/test_dfm_additive_overhang.md) |
| related | [test_dfm_cnc_undercut](/crates/oxide-geo-ops/src/dfm/test_dfm_cnc_undercut.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
