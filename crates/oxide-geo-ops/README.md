# `oxide-geo-ops`

> Kernel geometric operations: Booleans, Extrusions, Revolve, Fillet, Chamfer, and Direct Editing.

## Overview
`oxide-geo-ops` is an integral crate in the **Oxide-3D** industrial CAD/CAE/CAM/DCC ecosystem. 

Exact CSG boolean operations, parametric profile extrusions/revolutions, 2D sketch constraint solvers, and direct face offsets.

## Key Responsibilities
- Modular, memory-safe Rust 2024 implementation.
- Zero-cost abstractions adhering to `#![forbid(unsafe_code)]` boundaries where applicable.
- High performance, deterministic execution, and seamless integration with the Oxide-3D event-sourced architecture.

## Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
oxide-geo-ops = { path = "../../crates/oxide-geo-ops" }
```

## Architecture Context
Part of the unified Oxide-3D platform combining:
1. **Parametric CAD** (SolidWorks-style feature trees & B-Rep modeling)
2. **2D Drafting & OSNAP** (OpenCADStudio / AutoCAD parity)
3. **Procedural & Sculpt DCC** (Blender-style Geometry Nodes & Mesh Edit)
4. **Animation & VFX** (3ds Max-style track views & simulation)
