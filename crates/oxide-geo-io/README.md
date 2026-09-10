# `oxide-geo-io`

> CAD / DCC file format exchange (DXF, DWG, STEP, STL, OBJ, glTF).

## Overview
`oxide-geo-io` is an integral crate in the **Oxide-3D** industrial CAD/CAE/CAM/DCC ecosystem. 

Native DXF ASCII/binary parser and writer, DWG version sniffing (R14-2018), STEP AP242, and drawing crash recovery (.bak/.sv$).

## Key Responsibilities
- Modular, memory-safe Rust 2024 implementation.
- Zero-cost abstractions adhering to `#![forbid(unsafe_code)]` boundaries where applicable.
- High performance, deterministic execution, and seamless integration with the Oxide-3D event-sourced architecture.

## Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
oxide-geo-io = { path = "../../crates/oxide-geo-io" }
```

## Architecture Context
Part of the unified Oxide-3D platform combining:
1. **Parametric CAD** (SolidWorks-style feature trees & B-Rep modeling)
2. **2D Drafting & OSNAP** (OpenCADStudio / AutoCAD parity)
3. **Procedural & Sculpt DCC** (Blender-style Geometry Nodes & Mesh Edit)
4. **Animation & VFX** (3ds Max-style track views & simulation)
