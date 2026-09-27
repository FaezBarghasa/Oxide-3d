---
okf_version: "0.2"
type: Module
title: annotation
description: "CAD Dimensioning, Annotations, Multileaders, and Paper Space Layouts."
resource: crates/oxide-geo/src/annotation.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo/src/annotation
language: rust
---

# annotation

CAD Dimensioning, Annotations, Multileaders, and Paper Space Layouts.

## Docstring

CAD Dimensioning, Annotations, Multileaders, and Paper Space Layouts.

Provides production-ready data structures for:
- Linear, Aligned, Angular, Radial, and Diameter Dimensions
- `DimStyle` parameters (text height, arrow size, extension line offsets, tolerances)
- Multileader (`MLeader`) with leader lines, landing, and dogleg
- Paper Space Layouts (`PaperLayout`) with viewports (`Viewport2D`) and scaling (1:1, 1:50, 1:100)

## Relationships

| Type | Target |
|------|--------|
| related | [DimensionType](/crates/oxide-geo/src/annotation/DimensionType.md) |
| related | [DimStyle](/crates/oxide-geo/src/annotation/DimStyle.md) |
| related | [default](/crates/oxide-geo/src/annotation/default.md) |
| related | [default](/crates/oxide-geo/src/annotation/default.md) |
| related | [DimensionEntity](/crates/oxide-geo/src/annotation/DimensionEntity.md) |
| related | [measurement](/crates/oxide-geo/src/annotation/measurement.md) |
| related | [formatted_text](/crates/oxide-geo/src/annotation/formatted_text.md) |
| related | [measurement](/crates/oxide-geo/src/annotation/measurement.md) |
| related | [formatted_text](/crates/oxide-geo/src/annotation/formatted_text.md) |
| related | [MultiLeader](/crates/oxide-geo/src/annotation/MultiLeader.md) |
| related | [new](/crates/oxide-geo/src/annotation/new.md) |
| related | [new](/crates/oxide-geo/src/annotation/new.md) |
| related | [LayoutViewport](/crates/oxide-geo/src/annotation/LayoutViewport.md) |
| related | [PaperLayout](/crates/oxide-geo/src/annotation/PaperLayout.md) |
| related | [default](/crates/oxide-geo/src/annotation/default.md) |
| related | [default](/crates/oxide-geo/src/annotation/default.md) |
| related | [test_dimension_measurements_and_formatting](/crates/oxide-geo/src/annotation/test_dimension_measurements_and_formatting.md) |
| related | [test_paper_layout_defaults](/crates/oxide-geo/src/annotation/test_paper_layout_defaults.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
