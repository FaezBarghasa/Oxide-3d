# Changelog

All notable changes to the **Oxide-3D** industrial CAD/CAE/CAM/PLM engineering platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- **Production Web Application & Server (`apps/oxide-server`)**:
  - Embedded responsive single-page web app implementing the unified **3ds Max & OpenCADStudio** GUI/UX.
  - 13-Menu DCC top bar (File, Edit, Tools, Group, Views, Create, Modifiers, Animation, Graph Editors, Rendering, Customize, MaxScript, Help).
  - 4-Viewport Quad layout (Top Ortho, Front Ortho, Left Ortho, Perspective 3D) with interactive coordinate axes, grid, and toggleable maximization.
  - Right 6-tab 3ds Max Command Panel (Create, Modify, Hierarchy, Motion, Display, Utilities) with collapsible parameter rollouts.
  - Bottom animation timeline scrubber with transport controls (play, pause, step, keyframe).
  - OpenCADStudio command prompt with command history, drafting aliases (`LINE`/`L`, `CIRCLE`/`C`, `BOX`, `CYLINDER`, `PYRAMID`, `EXTRUDE`/`EXT`, `FILLET`, `BOM`, `GCODE`), live cursor coordinates, and drafting status tags (`SNAP`, `GRID`, `ORTHO`, `POLAR`, `OSNAP`, `MODEL`).
  - REST API endpoints for system health, exact B-Rep primitive generation (`/api/geometry/primitive`), CNC G-code postprocessing (`/api/cam/toolpath`), and multi-level BOM costing (`/api/plm/sample-bom`).
  - Automated visual verification and UI inspection using `playwright-cli`.
- **Bivariate Tensor-Product de Boor Algorithm (`crates/oxide-geo`)**:
  - Implemented exact bivariate tensor-product de Boor evaluation in homogeneous coordinates (`[wx, wy, wz, w]`) in [`surface.rs`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-geo/src/surface.rs), guaranteeing $C^\infty$ continuous analytical NURBS surface sampling.
- **Manifold Half-Edge Mesh Kernel (`crates/oxide-geo`)**:
  - Created [`half_edge.rs`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-geo/src/half_edge.rs) with typed `SlotMap` keys (`HeVertexKey`, `HalfEdgeKey`, `HeFaceKey`), topological triangle creation, and 1-ring neighbor traversal for Laplacian mesh smoothing.
- **Native `.oxd` Persistence Engine (`crates/oxide-persist`)**:
  - High-ratio Zstandard stream compression and MessagePack serialization for structured `OxdDocument<T>` and `OxdManifest` containers.
- **Settings & User Preferences (`crates/oxide-settings`)**:
  - Structured TOML configuration handling units, standards, grid spacing, snap precision, and dark mode under platform standard directories.
- **CAM Toolpaths & CNC Postprocessing (`crates/oxide-cam`)**:
  - 2.5D pocketing toolpath generation with axial stepdown and zigzag rasterization.
  - Multi-dialect postprocessors emitting ISO Fanuc, Haas, GRBL, and Siemens Sinumerik G-code programs.
- **Hierarchical PLM & Multi-Level BOM Costing (`crates/oxide-plm`)**:
  - Directed acyclic graph BOM tree computing aggregated quantities and extended cost rollups for complex assemblies.

---

## [0.1.0] - 2026-09-08

### Added
- **Heterogeneous Hardware Acceleration Layer (`oxide-hal`)**:
  - Unified asynchronous [`AcceleratorBackend`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-hal/src/lib.rs) trait supporting buffer management, asynchronous kernel dispatching, and synchronization.
  - Granular capability descriptor [`BackendCapabilities`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-hal/src/lib.rs) capturing double-precision (FP64), half-precision (FP16), unified memory, async compute queues, and SIMD warp sizes.
  - Multi-tier accelerator runtime orchestrator in [`oxide-compute-runtime`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-compute-runtime) with capability-weighted device scoring, automatic fallback, and Tokio task cancellation.
  - Dedicated isolated backend crates:
    - [`oxide-backend-cpu`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-backend-cpu): Rayon parallel multi-threaded CPU executor.
    - [`oxide-backend-wgpu`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-backend-wgpu): WebGPU / WGSL cross-platform compute.
    - [`oxide-backend-cuda`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-backend-cuda): NVIDIA CUDA hardware driver compute layer.
    - [`oxide-backend-rocm`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-backend-rocm): AMD ROCm / HIP compute backend.
    - [`oxide-backend-vulkan`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-backend-vulkan): Cross-platform Vulkan compute pipeline.
    - [`oxide-backend-metal`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-backend-metal): Apple Silicon Metal shading compute backend.
    - [`oxide-backend-dx12`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-backend-dx12): Windows Direct3D 12 DirectCompute backend.
    - [`oxide-backend-opencl`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-backend-opencl): OpenCL heterogeneous compute fallback.
  - Raw kernel sources stored in centralized repository directories (`kernels/cuda`, `kernels/hip`, `kernels/metal`, `kernels/wgsl`).
  - Safe FFI wrapper layer [`oxide-ffi-safe`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-ffi-safe) enforcing boundary memory safety.

- **Core Architecture & Event Engine (`oxide-core`, `oxide-math`)**:
  - Type-safe slotmap entity identifiers (`EntityKey`, `PartKey`, `FaceKey`, `EdgeKey`, `VertexKey`).
  - Immutable event-sourced command pipeline with [`OxideCommand`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-core/src/command.rs) and [`CommandBus`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-core/src/bus.rs).
  - Exact geometric predicates (Shewchuk-style `orient2d`, `orient3d`), robust `Interval` arithmetic, and configurable `Tolerance` models in [`oxide-math`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-math).

- **Geometry, Features & Procedural Graph**:
  - Full B-Rep topological structure (`Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid`) and `TopologyDatabase` in [`oxide-geo`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-geo).
  - High-level modeling operations (Extrude, Revolve, Fillet, Chamfer, Boolean CSG) in [`oxide-geo-ops`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-geo-ops).
  - Mesh and boundary exchange formats (STL, 3MF, glTF, STEP AP242 scaffolding) in [`oxide-geo-io`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-geo-io).
  - Parametric Feature Tree with topological naming maps in [`oxide-feature`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-feature).
  - High-performance Procedural Geometry Nodes engine in [`oxide-nodes`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-nodes).

- **Simulation & Multiphysics Solvers (CAE)**:
  - Unified simulation domain types and boundary conditions in [`oxide-sim-core`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-sim-core).
  - Linear static and modal Finite Element Analysis (FEA) engine powered by `faer` in [`oxide-sim-fea`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-sim-fea).
  - D3Q19 Lattice Boltzmann Method (LBM) Computational Fluid Dynamics solver in [`oxide-sim-cfd`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-sim-cfd).
  - Solid Isotropic Material with Penalization (SIMP) Topology Optimization engine in [`oxide-sim-topopt`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-sim-topopt).

- **Assembly, Mechanism & Manufacturing (CAM / Metrology)**:
  - 100,000+ interactive assembly scene graph with spatial BVH acceleration in [`oxide-scene`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-scene).
  - Real-time kinematic simulation with rigid body physics via `rapier3d` in [`oxide-mech`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-mech).
  - 2.5D and 3D adaptive pocketing, contouring, and multi-axis G-code postprocessing in [`oxide-cam`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-cam).
  - ASME Y14.5 and ISO 1101 compliant Geometric Dimensioning & Tolerancing (GD&T) engine in [`oxide-metrology`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-metrology).

- **Data Persistence, PLM & Real-Time Collaboration**:
  - Embedded high-performance transactional PLM storage powered by `redb` in [`oxide-plm`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-plm).
  - Zero-copy `.oxd` container storage format with Zstandard compression in [`oxide-persist`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-persist).
  - Multi-user conflict-free replicated data type (CRDT) document synchronization via `automerge` in [`oxide-collab`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-collab).

- **Automation, Scripting & Extension Ecosystem**:
  - Macro recorder and headless workflow engine in [`oxide-automation`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-automation).
  - Sandboxed Rhai scripting engine with CAD API bindings in [`oxide-script-rhai`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-script-rhai).
  - Python runtime bridge using PyO3 in [`oxide-script-python`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-script-python).
  - Sandboxed WebAssembly (Wasm) plugin execution host with `wasmtime` in [`oxide-plugins-wasm`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-plugins-wasm) and official plugin SDK [`plugins/sdk`](file:///home/jrad/RustroverProjects/Oxide-3d/plugins/sdk).

- **User Interface & Application Binaries**:
  - Desktop CAD application [`apps/oxide-desktop`](file:///home/jrad/RustroverProjects/Oxide-3d/apps/oxide-desktop) built on the Iced MVU GUI architecture.
  - Headless batch processing and rendering CLI [`apps/oxide-headless`](file:///home/jrad/RustroverProjects/Oxide-3d/apps/oxide-headless).
  - High-throughput collaboration and simulation server [`apps/oxide-server`](file:///home/jrad/RustroverProjects/Oxide-3d/apps/oxide-server).
  - Developer automation toolkit [`tools/xtask`](file:///home/jrad/RustroverProjects/Oxide-3d/tools/xtask).

- **Project Documentation**:
  - [`README.md`](file:///home/jrad/RustroverProjects/Oxide-3d/README.md): High-level system overview, benchmark goals, crate ecosystem map, and build instructions.
  - [`docs/architecture.md`](file:///home/jrad/RustroverProjects/Oxide-3d/docs/architecture.md): Deep architectural specification covering HAL, event sourcing, MVU decoupling, memory safety, and thread boundaries.
  - [`docs/design.md`](file:///home/jrad/RustroverProjects/Oxide-3d/docs/design.md): Domain modeling and mathematical formalisms for B-Rep topology, FEA, CFD, and CRDT synchronization.
  - [`docs/app_flow.md`](file:///home/jrad/RustroverProjects/Oxide-3d/docs/app_flow.md): End-to-end lifecycle flows for desktop UI, headless CLI automation, and cloud server execution.
  - [`docs/tdd.md`](file:///home/jrad/RustroverProjects/Oxide-3d/docs/tdd.md): Test-Driven Development protocols, unit testing benchmarks, and CI validation suites.
  - [`docs/llm_context.md`](file:///home/jrad/RustroverProjects/Oxide-3d/docs/llm_context.md): System prompt guidelines and architectural invariants for AI-assisted engineering.
