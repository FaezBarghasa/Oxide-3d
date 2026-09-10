# Oxide-3D — Unified Industrial CAD/CAE/CAM & DCC Platform

High-performance, GPU-accelerated, event-sourced engineering and digital creation operating system built in modern Rust (2024 edition).

---

## 🌟 The Unified Paradigm

Oxide-3D unifies three foundational creative and engineering disciplines into a single memory-safe, hardware-accelerated architecture:

1. **Parametric CAD & Precision Drafting (SolidWorks & OpenCADStudio Parity)**
   - Exact analytical B-Rep boundary representation (`Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid`).
   - 2D Drafting Engine with Line, Polyline, Arc, Circle, Ellipse, Spline, Hatch, and Text entities.
   - 11-mode Object Snap (OSNAP) detection: Endpoint, Midpoint, Center, Geometric Center, Quadrant, Intersection, Perpendicular, Tangent, Nearest, Parallel.
   - AutoCAD-style interactive command prompt with 30+ aliases (`L`, `PL`, `C`, `A`, `REC`, `M`, `CO`, `RO`, `SC`, `TR`, `EX`, `EXT`, `REV`, `UNI`).
   - Native DXF ASCII/binary parsing and generation, DWG version sniffing (R14–R2018), and crash recovery (`.bak`/`.sv$`).
   - Associative dimensions, multileaders (`MLEADER`), paper space layouts, and viewport scale locking (`MVIEW`).

2. **Procedural & Sculpt DCC (Blender Parity)**
   - Procedural Geometry Nodes computational graph engine with field attribute evaluation.
   - Non-destructive modifier stack (Subdivision, Mirror, Bevel, Array, Boolean, Deform).
   - Mesh Edit mode and dynamic topology (Dyntopo) sculpting brushes.
   - Dual topology bridge tessellating exact B-Rep solids to GPU triangle meshes in real-time.

3. **Animation, VFX & Simulation (3ds Max & Engineering Parity)**
   - Keyframe controllers (Bezier, Linear, TCB), track view curve editors, and animation timeline evaluation.
   - Physics & particle systems (MassFX rigid bodies, space warps, forces, and deflectors).
   - FEA linear static stress/modal solvers, GPU Lattice Boltzmann Method (LBM) CFD, and SIMP topology optimization.
   - Native Model Context Protocol (MCP) server for external AI agent automation and tool dispatching.

---

## 🏗️ Workspace Architecture (41 Modular Crates)

- **UI Shell (`apps/oxide-desktop`, `crates/oxide-ui`, `crates/oxide-ui-widgets`)**: Non-blocking Iced MVU dockable workspace with Drafting Ribbon, DCC Command Panel, CAD CommandManager, and 24-slot Material Editor.
- **Core & History (`crates/oxide-core`, `crates/oxide-math`)**: Immutable event-sourced operation log with UUIDv7 tracking, infinite undo/redo, rollback bar sliding, and SIMD linear algebra (`glam`/`nalgebra`).
- **Geometry & CAD Kernel (`crates/oxide-geo`, `crates/oxide-geo-ops`, `crates/oxide-geo-io`)**: Exact B-Rep topology database, CSG Booleans, variational sketch constraint solvers, profile extrusions/revolutions, and DXF/DWG/STEP I/O.
- **Scene & Nodes (`crates/oxide-scene`, `crates/oxide-nodes`, `crates/oxide-feature`)**: Flat ECS-style scene container with BVH spatial indexing, Geometry Nodes attribute DAG, and SolidWorks-style feature trees.
- **Rendering & Shaders (`crates/oxide-render`, `crates/oxide-backend-wgpu`)**: `wgpu` Omniviewport renderer with PBR shading, CAD technical wireframes, and GPU picking.
- **Compute Backends (`crates/oxide-backend-*`, `crates/oxide-hal`, `crates/oxide-compute`)**: Heterogeneous hardware acceleration across CUDA, ROCm, Vulkan, Metal, DX12, OpenCL, and CPU Rayon/SIMD.
- **Simulation Fabric (`crates/oxide-sim-core`, `crates/oxide-sim-fea`, `crates/oxide-sim-cfd`, `crates/oxide-sim-topopt`)**: High-performance FEA, real-time CFD, and generative design.
- **CAM, Mechanics & PLM (`crates/oxide-cam`, `crates/oxide-mech`, `crates/oxide-metrology`, `crates/oxide-plm`)**: Toolpaths, kinematics mates, GD&T inspection, and BOM revision management.
- **Automation & Extensibility (`crates/oxide-automation`, `crates/oxide-script-python`, `crates/oxide-script-rhai`, `crates/oxide-plugins-wasm`, `crates/oxide-collab`)**: JSON-RPC 2.0 MCP automation server, PyO3 Python bindings, Rhai macros, Wasm sandboxed plugins, and CRDT multiplayer sync.

---

## 🛠️ Building, Testing & Verification

```bash
# Check code formatting
cargo fmt --check

# Run all workspace unit and doc tests
cargo test --workspace

# Launch the desktop UI application
cargo run -p oxide-desktop

# Launch headless automation / CLI export
cargo run -p oxide-headless -- --export drawing.dwg output.dxf

# Launch the MCP server over stdio for AI agent control
cargo run -p oxide-desktop -- --mcp
```
