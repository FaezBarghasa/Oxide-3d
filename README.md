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

## 🏗️ Workspace Architecture (45 Modular Crates & 3 Applications)

- **Applications Layer (`apps/`)**:
  - `apps/oxide-desktop`: Non-blocking Iced MVU desktop GUI application with dockable workspaces, CAD CommandManager, DCC Command Panel, and Drafting command prompt.
  - `apps/oxide-server`: Full-featured production web platform serving the interactive 3ds Max & OpenCADStudio Web UI, REST API for exact B-Rep geometry, CAM G-code generation, and PLM BOM inspection. Supports automated Playwright visual testing.
  - `apps/oxide-headless`: Command-line automation and CI runner for headless CAD/CAE/CAM processing and format export.
- **UI Shell (`crates/oxide-ui`, `crates/oxide-ui-widgets`)**: 13-Menu DCC top bar, 6-tab Command Panel with collapsible rollouts, 4-viewport quad layout, timeline animation scrubber, and AutoCAD-style command prompt with 30+ aliases.
- **Core & History (`crates/oxide-core`, `crates/oxide-math`, `crates/oxide-settings`)**: Immutable event-sourced operation log with UUIDv7 tracking, infinite undo/redo, rollback bar sliding, SIMD linear algebra (`glam`/`nalgebra`), and persistent TOML settings.
- **Geometry & CAD Kernel (`crates/oxide-geo`, `crates/oxide-geo-ops`, `crates/oxide-geo-io`)**: Exact analytical B-Rep boundary representation, bivariate tensor-product de Boor NURBS surface evaluator, manifold half-edge mesh kernel with Laplacian smoothing, CSG Booleans, variational sketch constraint solvers, profile extrusions/revolutions, and DXF/DWG/STEP I/O.
- **Persistence & Storage (`crates/oxide-persist`)**: Native `.oxd` format container using high-ratio Zstandard compression and MessagePack binary serialization.
- **Scene & Nodes (`crates/oxide-scene`, `crates/oxide-nodes`, `crates/oxide-feature`)**: Flat ECS-style scene container with BVH spatial indexing, Geometry Nodes attribute DAG, and SolidWorks-style feature trees.
- **Rendering & Shaders (`crates/oxide-render`, `crates/oxide-backend-wgpu`)**: `wgpu` Omniviewport renderer with PBR shading, CAD technical wireframes, and GPU picking.
- **Compute Backends (`crates/oxide-backend-*`, `crates/oxide-hal`, `crates/oxide-compute`)**: Heterogeneous hardware acceleration across CUDA, ROCm, Vulkan, Metal, DX12, OpenCL, and CPU Rayon/SIMD.
- **Simulation Fabric (`crates/oxide-sim-core`, `crates/oxide-sim-fea`, `crates/oxide-sim-cfd`, `crates/oxide-sim-topopt`)**: High-performance FEA, real-time CFD, and generative design.
- **CAM, Mechanics & PLM (`crates/oxide-cam`, `crates/oxide-mech`, `crates/oxide-metrology`, `crates/oxide-plm`)**: 2.5D pocketing toolpaths, Fanuc/Haas/GRBL/Siemens G-code postprocessors, kinematic joints, GD&T inspection, and recursive multi-level BOM costing.
- **Automation & Extensibility (`crates/oxide-automation`, `crates/oxide-script-python`, `crates/oxide-script-rhai`, `crates/oxide-plugins-wasm`, `crates/oxide-collab`)**: JSON-RPC 2.0 MCP automation server, PyO3 Python bindings, Rhai macros, Wasm sandboxed plugins, and CRDT multiplayer sync.

---

## 🛠️ Building, Testing & Verification

```bash
# Check code formatting
cargo fmt --check

# Run all workspace unit and doc tests across all 45 crates
cargo test --workspace

# Launch the desktop UI application
cargo run -p oxide-desktop

# Launch the web platform & REST server (http://localhost:8080)
cargo run -p oxide-server

# Perform automated visual examination via Playwright CLI
playwright-cli open http://127.0.0.1:8080
playwright-cli screenshot

# Launch headless automation / CLI export
cargo run -p oxide-headless -- open drawing.oxd --export-step output.step

# Launch the MCP server over stdio for AI agent control
cargo run -p oxide-desktop -- --mcp
```
