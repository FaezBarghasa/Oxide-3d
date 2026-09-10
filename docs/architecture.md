# Oxide-3D — System Architecture

Comprehensive engineering overview of the **Oxide-3D** industrial CAD/CAE/CAM/DCC monorepo built in modern Rust (2024 edition).

---

## 1. High-Level Architectural Model

Oxide-3D adopts an **Event-Sourced Document & Command Bus Model** unifying Parametric CAD (SolidWorks/OpenCADStudio), Procedural DCC (Blender), and Animation/VFX (3ds Max) with strict separation between the asynchronous UI thread, Rayon CPU parallelism, and multi-backend hardware accelerators.

```text
+-----------------------------------------------------------------------------------+
|                                 Applications Layer                                |
|   apps/oxide-desktop (Iced GUI)   |   apps/oxide-headless (CLI CI)  | oxide-server |
+------------------------------------------+----------------------------------------+
                                           |
                                           v
+-----------------------------------------------------------------------------------+
|                                 Domain Command Bus                                |
|   crates/oxide-core (Commands, Events, IDs, Units) | crates/oxide-automation (MCP)     |
+--------------------+---------------------+---------------------+------------------+
                     |                     |                     |
                     v                     v                     v
+--------------------------+ +-------------------------+ +--------------------------+
|  CAD & Drafting Engine   | |     DCC & Assembly      | |     Simulation Core      |
| oxide-geo (B-Rep, OSNAP) | | oxide-scene (ECS, BVH)  | | oxide-sim-core (Meshes)  |
| oxide-geo-ops (Booleans) | | oxide-render (wgpu PBR) | | oxide-sim-fea (faer CG)  |
| oxide-geo-io (DWG/DXF)   | | oxide-nodes (GeoNodes)  | | oxide-sim-cfd (LBM/FVM)  |
| oxide-feature (DAG tree) | | oxide-mech (Rapier3d)   | | oxide-sim-topopt (SIMP)  |
| oxide-cam (G-code)       | | oxide-metrology (GD&T)  | | oxide-plm (redb Items)   |
+--------------------------+ +-------------------------+ +--------------------------+
                                           |
                                           v
+-----------------------------------------------------------------------------------+
|                        oxide-compute-runtime & oxide-hal                          |
|   Hardware Probing | Capability Scoring | Requirement Matching | Cancellation    |
+----------+------------+------------+------------+------------+-----------+--------+
           |            |            |            |            |           |
           v            v            v            v            v           v
    +-------------+ +--------+ +-----------+ +---------+ +----------+ +--------+
    | NVIDIA CUDA | |AMD ROCm| |Vulkan Comp| |App Metal| |Direct3D12| |Host CPU |
    | cuBLAS/SPARSE |rocBLAS | | SPIR-V    | | MSL     | | HLSL CS  | | Rayon  |
    +-------------+ +--------+ +-----------+ +---------+ +----------+ +--------+
```

---

## 2. Core Pillars & Multi-Paradigm Unification

### 2.1 Event-Sourced Immutable Operation Log (`oxide-core::event_log`)
Every design action is captured as an `OperationRecord` with millisecond UUIDv7 keys in an append-only log:
- **Deterministic Replay**: Re-evaluates parametric models and modifier stacks identically across systems.
- **Rollback Bar & Infinite Undo/Redo**: Rollback to any step $k$ in the timeline without mutating fragile pointers.
- **Unified Multi-Paradigm Operations**: Handles CAD features (Extrude, Revolve, Fillet), DCC modifiers (Subdivision, Bevel, Geometry Nodes), and Animation keyframes in one chronological stream.

### 2.2 Dual-Topology Engine (`oxide-geo` & `oxide-geo-ops`)
- **Exact Analytical B-Rep**: Boundary representation (`Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid`) for high-precision parametric CAD modeling.
- **2D Drafting & OSNAP**: High-performance 2D CAD drafting with 11-mode precision snapping (Endpoint, Midpoint, Center, Quadrant, Intersection, Tangent, Perpendicular, etc.) and native DWG/DXF exchange.
- **Polygonal Tessellation Bridge**: Real-time adaptive tessellation syncing B-Rep solids to GPU triangle meshes for DCC sculpting and PBR rendering.

### 2.3 Data-Oriented Assembly ECS (`oxide-scene`)
- Uses `slotmap` keys (`EntityKey`, `PartKey`, `FaceKey`, `EdgeKey`, `VertexKey`) for $O(1)$ cache-friendly lookup.
- Capable of streaming and managing 100,000+ parts at 60 FPS viewport frame rates with spatial BVH acceleration.

### 2.4 Heterogeneous Compute Acceleration (`oxide-compute-runtime` & `oxide-hal`)
- **CUDA & ROCm**: Double-precision (FP64) sparse solvers for FEA and high-speed LBM CFD.
- **Metal & Vulkan**: GPU viewport rendering, GPU color picking, and real-time topology optimization isosurfaces.
- **CPU Parallel Fallback**: Rayon multi-threading with SIMD for exact CAD boolean operations.

### 2.5 Native Model Context Protocol (MCP) Automation (`oxide-automation`)
- Embedded JSON-RPC 2.0 MCP server over stdio for external AI agents.
- Tools for automated geometric creation, drawing queries, layer setup, and format exports.

### 2.6 Persistence & Binary Container Format (`oxide-persist`)
- Native `.oxd` file container utilizing MessagePack binary serialization (`rmp-serde`) and high-ratio Zstandard compression (`zstd`).
- Structured `OxdDocument<T>` carrying schema-versioned `OxdManifest` metadata, scene graph state, and B-Rep topological definitions.

### 2.7 Automated CNC Toolpath & G-Code Generation (`oxide-cam`)
- 2.5D pocketing toolpath generation with axial stepdown passes and bidirectional zigzag rasterization.
- Multi-dialect postprocessors emitting standard ISO G-code programs for Fanuc, Haas, GRBL, and Siemens Sinumerik controllers.

### 2.8 Product Lifecycle Management & Multi-Level BOM Costing (`oxide-plm`)
- Directed acyclic graph BOM tree tracking multi-level assembly hierarchies.
- Automated recursive accumulation algorithm calculating total part quantities and extended cost rollups across complex engineering assemblies.

### 2.9 Unified 3ds Max & OpenCADStudio GUI/UX (`apps/oxide-server` & `apps/oxide-desktop`)
- Single unified interface offering:
  - 13-Menu DCC top bar (File, Edit, Tools, Group, Views, Create, Modifiers, Animation, Graph Editors, Rendering, Customize, MaxScript, Help).
  - 4-Viewport Quad layout (Top Ortho, Front Ortho, Left Ortho, Perspective 3D) with interactive coordinate axes, grid, and maximization toggle.
  - 6-Tab 3ds Max Command Panel with collapsible parameter rollouts.
  - Timeline animation scrubber and keyframe controllers.
  - OpenCADStudio command prompt with command history, drafting aliases (`LINE`/`L`, `CIRCLE`/`C`, `BOX`, `CYLINDER`, `PYRAMID`, `EXTRUDE`/`EXT`, `FILLET`, `BOM`, `GCODE`), live cursor coordinates, and drafting status tags.
  - Full automated visual verification via `playwright-cli`.
