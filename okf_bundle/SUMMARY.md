---
description: 'Top-level OKF summary: 2846 concepts across 4 domains and 103 modules'
git_branch: master
git_repo: Oxide-3d
okf_version: '0.2'
timestamp: '2026-09-27T20:34:24Z'
title: Oxide-3d — Knowledge Summary
type: Index
---

# Oxide-3d — Knowledge Summary

> OKF v0.2 bundle | 2,846 concepts | 4 domains | 103 modules

## Stats

| Type | Count |
|------|-------|
| Dependency | 1,324 |
| Function | 1,105 |
| Class | 314 |
| Module | 103 |

| Language | Concepts |
|----------|----------|
| rust | 1,522 |
| manifest | 1,324 |

## Domain Map

Use these links to navigate the bundle or prime an AI agent with focused context.

### [apps](apps/index.md) — 26 concepts

- [apps/oxide-server/src/handlers](apps/oxide-server/src/handlers/index.md) (16 concepts) — REST API Handlers for Oxide-3D Web Platform.
- [apps/oxide-headless/src/main](apps/oxide-headless/src/main/index.md) (4 concepts) — Oxide-3D Headless Automation & CI Runner.
- [apps/oxide-desktop/src/main](apps/oxide-desktop/src/main/index.md) (2 concepts) — Oxide-3D Desktop GUI Application Entry Point.
- [apps/oxide-server/src/main](apps/oxide-server/src/main/index.md) (2 concepts) — Oxide-3D Web & Collaboration Server.
- [apps/oxide-server/src/web_ui](apps/oxide-server/src/web_ui/index.md) (2 concepts) — Production Web Client for Oxide-3D CAD/DCC Platform.

### [crates](crates/index.md) — 1,490 concepts

- [crates/oxide-nodes/src/lib](crates/oxide-nodes/src/lib/index.md) (129 concepts) — Oxide-3D Procedural Geometry Nodes Dataflow Engine.
- [crates/oxide-ui/src/dcc_menu](crates/oxide-ui/src/dcc_menu/index.md) (52 concepts) — Complete 13-Menu Bar System for DCC / 3D Digital Content Creation in Oxide-3D.
- [crates/oxide-math/src/interval](crates/oxide-math/src/interval/index.md) (45 concepts)
- [crates/oxide-geo-ops/src/solver_3d](crates/oxide-geo-ops/src/solver_3d/index.md) (43 concepts) — 3D Variational Geometric & Assembly Constraint Solver.
- [crates/oxide-geo/src/topology](crates/oxide-geo/src/topology/index.md) (40 concepts) — B-Rep Topology entities (Vertex, Edge, Wire, Face, Shell, Solid) and storage are
- [crates/oxide-ui/src/command_panel](crates/oxide-ui/src/command_panel/index.md) (38 concepts) — 6-Tab Command Panel Model for Oxide-3D DCC System.
- [crates/oxide-math/src/tolerance](crates/oxide-math/src/tolerance/index.md) (31 concepts)
- [crates/oxide-core/src/event_log](crates/oxide-core/src/event_log/index.md) (28 concepts) — Event-Sourced Document and Temporal Operation Log for Oxide-3D.
- *…and 88 more modules*

### [plugins](plugins/index.md) — 2 concepts

- [plugins/sdk/src/lib](plugins/sdk/src/lib/index.md) (2 concepts) — Oxide-3D Plugin Authoring SDK for WebAssembly Components.

### [tools](tools/index.md) — 4 concepts

- [tools/xtask/src/main](tools/xtask/src/main/index.md) (4 concepts) — Workspace automation tasks for Oxide-3D.

## Dependencies

> Full list at [`_dependencies/index.md`](/_dependencies/index.md) or `okf lookup --type Dependency`

| Ecosystem | Packages |
|----------|----------|
| cargo | 1,324 |

## Key Concepts

Highest-value concepts across all domains (Classes and Functions with rich descriptions).

| Concept | Type | Module | Description |
|---------|------|--------|-------------|
| [update_densities](/crates/oxide-sim-topopt/src/lib/update_densities.md) | Function | `crates/oxide-sim-topopt/src` | Perform an Optimality Criteria (OC) density update step give… |
| [update_densities](/crates/oxide-sim-topopt/src/lib/update_densities_1.md) | Function | `crates/oxide-sim-topopt/src` | Perform an Optimality Criteria (OC) density update step give… |
| [export_dxf](/crates/oxide-geo-ops/src/drawing_sheet/export_dxf.md) | Function | `crates/oxide-geo-ops/src` | Export drawing sheet to open AutoCAD DXF (R12 / AC1009) form… |
| [export_dxf](/crates/oxide-geo-ops/src/drawing_sheet/export_dxf_1.md) | Function | `crates/oxide-geo-ops/src` | Export drawing sheet to open AutoCAD DXF (R12 / AC1009) form… |
| [OperationId](/crates/oxide-core/src/id/OperationId.md) | Class | `crates/oxide-core/src` | A universally unique operation identifier based on UUIDv7 fo… |
| [append](/crates/oxide-core/src/event_log/append.md) | Function | `crates/oxide-core/src` | Appends and applies a new operation to the log, truncating a… |
| [append](/crates/oxide-core/src/event_log/append_1.md) | Function | `crates/oxide-core/src` | Appends and applies a new operation to the log, truncating a… |
| [catch_ffi_boundary](/crates/oxide-ffi-safe/src/lib/catch_ffi_boundary.md) | Function | `crates/oxide-ffi-safe/src` | Safely execute an FFI block while catching potential unhandl… |
| [analyze_additive](/crates/oxide-geo-ops/src/dfm/analyze_additive.md) | Function | `crates/oxide-geo-ops/src` | Analyze mesh for 3D Printing / Additive issues (overhangs re… |
| [analyze_additive](/crates/oxide-geo-ops/src/dfm/analyze_additive_1.md) | Function | `crates/oxide-geo-ops/src` | Analyze mesh for 3D Printing / Additive issues (overhangs re… |
| [solve_linear_static](/crates/oxide-sim-fea/src/lib/solve_linear_static.md) | Function | `crates/oxide-sim-fea/src` | Solves a linear static stress analysis problem using Faer de… |
| [ComputeError](/crates/oxide-compute/src/traits/ComputeError.md) | Class | `crates/oxide-compute/src` | Errors arising from compute device discovery, buffer allocat… |
| [NativeComputeDevice](/crates/oxide-compute/src/native/NativeComputeDevice.md) | Class | `crates/oxide-compute/src` | Generic native accelerator device wrapper providing unified … |
| [calculate_bom_rollup](/crates/oxide-plm/src/lib/calculate_bom_rollup.md) | Function | `crates/oxide-plm/src` | Recursively calculate total aggregated bill of materials rol… |
| [calculate_bom_rollup](/crates/oxide-plm/src/lib/calculate_bom_rollup_1.md) | Function | `crates/oxide-plm/src` | Recursively calculate total aggregated bill of materials rol… |
| [KernelRegistry](/crates/oxide-kernels/src/lib/KernelRegistry.md) | Class | `crates/oxide-kernels/src` | Central registry mapping engineering operations to hardware-… |
| [evaluate_true_position](/crates/oxide-metrology/src/lib/evaluate_true_position.md) | Function | `crates/oxide-metrology/src` | Evaluate True Position (RFS) for measured center points agai… |
| [evaluate_true_position](/crates/oxide-metrology/src/lib/evaluate_true_position_1.md) | Function | `crates/oxide-metrology/src` | Evaluate True Position (RFS) for measured center points agai… |
| [from_translation_rotation](/crates/oxide-geo-ops/src/solver_3d/from_translation_rotation.md) | Function | `crates/oxide-geo-ops/src` | Construct from translation vector [tx, ty, tz] and unit quat… |
| [from_translation_rotation](/crates/oxide-geo-ops/src/solver_3d/from_translation_rotation_1.md) | Function | `crates/oxide-geo-ops/src` | Construct from translation vector [tx, ty, tz] and unit quat… |

## Usage with OpenCode

```bash
# Prime full context
RUN cat ./okf_bundle/SUMMARY.md

# Prime specific domain
RUN cat ./okf_bundle/apps/index.md

# Find a concept
RUN find ./okf_bundle -name '<ConceptName>.md' | xargs cat
```
