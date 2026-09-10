# `oxide-render`

> GPU-accelerated viewport renderer (wgpu), PBR materials, wireframe, and picking.

## Overview
`oxide-render` is an integral crate in the **Oxide-3D** industrial CAD/CAE/CAM/DCC ecosystem. 

CAD technical illustration rendering, shaded PBR lighting, viewport gizmos, instanced line drawing, and GPU mouse picking.

## Key Responsibilities
- Modular, memory-safe Rust 2024 implementation.
- Zero-cost abstractions adhering to `#![forbid(unsafe_code)]` boundaries where applicable.
- High performance, deterministic execution, and seamless integration with the Oxide-3D event-sourced architecture.

## Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
oxide-render = { path = "../../crates/oxide-render" }
```

## Architecture Context
Part of the unified Oxide-3D platform combining:
1. **Parametric CAD** (SolidWorks-style feature trees & B-Rep modeling)
2. **2D Drafting & OSNAP** (OpenCADStudio / AutoCAD parity)
3. **Procedural & Sculpt DCC** (Blender-style Geometry Nodes & Mesh Edit)
4. **Animation & VFX** (3ds Max-style track views & simulation)
