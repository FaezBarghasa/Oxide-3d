---
okf_version: "0.2"
type: Module
title: cad_dxf_dwg
description: DXF and DWG CAD Interchange and Recovery Engine.
resource: crates/oxide-geo-io/src/cad_dxf_dwg.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-geo-io"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-io/src/cad_dxf_dwg
language: rust
---

# cad_dxf_dwg

DXF and DWG CAD Interchange and Recovery Engine.

## Docstring

DXF and DWG CAD Interchange and Recovery Engine.

Provides precision export and import for:
- AutoCAD DXF (R12 through R2018 format)
- Drawing Layers, Colors (ACI index & RGB), Line types
- 2D Entities: LINE, LWPOLYLINE, CIRCLE, ARC, ELLIPSE, HATCH, TEXT/MTEXT
- Binary DWG Header and Metadata detection
- Autosave (`.sv$`) and Backup (`.bak`) recovery pipelines

## Relationships

| Type | Target |
|------|--------|
| related | [DxfPair](/crates/oxide-geo-io/src/cad_dxf_dwg/DxfPair.md) |
| related | [new](/crates/oxide-geo-io/src/cad_dxf_dwg/new.md) |
| related | [new](/crates/oxide-geo-io/src/cad_dxf_dwg/new.md) |
| related | [DxfCodec](/crates/oxide-geo-io/src/cad_dxf_dwg/DxfCodec.md) |
| related | [new](/crates/oxide-geo-io/src/cad_dxf_dwg/new.md) |
| related | [export_dxf](/crates/oxide-geo-io/src/cad_dxf_dwg/export_dxf.md) |
| related | [import_dxf](/crates/oxide-geo-io/src/cad_dxf_dwg/import_dxf.md) |
| related | [new](/crates/oxide-geo-io/src/cad_dxf_dwg/new.md) |
| related | [export_dxf](/crates/oxide-geo-io/src/cad_dxf_dwg/export_dxf.md) |
| related | [import_dxf](/crates/oxide-geo-io/src/cad_dxf_dwg/import_dxf.md) |
| related | [DwgVersion](/crates/oxide-geo-io/src/cad_dxf_dwg/DwgVersion.md) |
| related | [from_magic](/crates/oxide-geo-io/src/cad_dxf_dwg/from_magic.md) |
| related | [from_magic](/crates/oxide-geo-io/src/cad_dxf_dwg/from_magic.md) |
| related | [DwgSniffer](/crates/oxide-geo-io/src/cad_dxf_dwg/DwgSniffer.md) |
| related | [inspect](/crates/oxide-geo-io/src/cad_dxf_dwg/inspect.md) |
| related | [inspect](/crates/oxide-geo-io/src/cad_dxf_dwg/inspect.md) |
| related | [CadRecoveryManager](/crates/oxide-geo-io/src/cad_dxf_dwg/CadRecoveryManager.md) |
| related | [default](/crates/oxide-geo-io/src/cad_dxf_dwg/default.md) |
| related | [default](/crates/oxide-geo-io/src/cad_dxf_dwg/default.md) |
| related | [new](/crates/oxide-geo-io/src/cad_dxf_dwg/new.md) |
| related | [make_bak_file](/crates/oxide-geo-io/src/cad_dxf_dwg/make_bak_file.md) |
| related | [make_autosave](/crates/oxide-geo-io/src/cad_dxf_dwg/make_autosave.md) |
| related | [new](/crates/oxide-geo-io/src/cad_dxf_dwg/new.md) |
| related | [make_bak_file](/crates/oxide-geo-io/src/cad_dxf_dwg/make_bak_file.md) |
| related | [make_autosave](/crates/oxide-geo-io/src/cad_dxf_dwg/make_autosave.md) |
| related | [test_dxf_export_and_import_roundtrip](/crates/oxide-geo-io/src/cad_dxf_dwg/test_dxf_export_and_import_roundtrip.md) |
| related | [test_dwg_version_detection](/crates/oxide-geo-io/src/cad_dxf_dwg/test_dwg_version_detection.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
