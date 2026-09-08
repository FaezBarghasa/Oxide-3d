# Changelog

All notable changes to the **Oxide-3D** industrial CAD/CAE/CAM/PLM engineering platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned / In Progress
- Concrete native driver dynamic linking bridges for `cudarc` (CUDA), `ash` (Vulkan), and `metal-rs` (Metal).
- Native STEP AP242 ISO 10303 reader/writer integration in [`oxide-geo-io`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-geo-io).
- Direct interactive viewport canvas bridge between [`oxide-ui-widgets`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-ui-widgets) and [`oxide-render`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-render) in Iced MVU.
- Advanced sparse Cholesky / Conjugate Gradient linear equation solvers in [`oxide-sim-fea`](file:///home/jrad/RustroverProjects/Oxide-3d/crates/oxide-sim-fea) via `faer`.

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
