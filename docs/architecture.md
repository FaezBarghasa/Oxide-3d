# Oxide-3D — System Architecture

Comprehensive engineering overview of the **Oxide-3D** industrial CAD/CAE/CAM/PLM monorepo built in modern Rust (2024 edition).

---

## 1. High-Level Architectural Model

Oxide-3D adopts an **Event-Sourced Document & Command Bus Model** with strict separation between the asynchronous UI thread, Rayon CPU parallelism, and multi-backend hardware accelerators.

```text
+-----------------------------------------------------------------------------------+
|                                 Applications Layer                                |
|   apps/oxide-desktop (Iced GUI)   |   apps/oxide-headless (CLI CI)  | oxide-server |
+------------------------------------------+----------------------------------------+
                                           |
                                           v
+-----------------------------------------------------------------------------------+
|                                 Domain Command Bus                                |
|   crates/oxide-core (Commands, Events, IDs, Units) | crates/oxide-automation (Macros)  |
+--------------------+---------------------+---------------------+------------------+
                     |                     |                     |
                     v                     v                     v
+--------------------------+ +-------------------------+ +--------------------------+
|      Geometry Core       | |     Assembly Scene      | |     Simulation Core      |
| oxide-geo (B-Rep DB)     | | oxide-scene (ECS, BVH)  | | oxide-sim-core (Meshes)  |
| oxide-geo-ops (Booleans) | | oxide-render (wgpu PBR) | | oxide-sim-fea (faer CG)  |
| oxide-geo-io (STEP/3MF)  | | oxide-mech (Rapier3d)   | | oxide-sim-cfd (LBM/FVM)  |
| oxide-nodes (Geometry)   | | oxide-cam (G-code)      | | oxide-sim-topopt (SIMP)  |
| oxide-feature (DAG tree) | | oxide-metrology (GD&T)  | | oxide-plm (redb Items)   |
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

## 2. Core Pillars

### 2.1 Event-Sourced Immutable Operation Log
Every design action is captured as an `OxideCommand` and stored in an append-only operation log:
- **Deterministic Replay**: Re-evaluates models identically across operating systems.
- **Git-Native 3D Versioning**: Operations merge cleanly with CRDT algorithms (`automerge`).
- **Instant Undo/Redo**: History navigation without mutating fragile document pointers.

### 2.2 Data-Oriented Assembly ECS
- Uses `slotmap` keys (`EntityKey`, `PartKey`, `FaceKey`, `EdgeKey`, `VertexKey`) for $O(1)$ lookup.
- Capable of streaming and managing 100,000+ parts at 60 FPS viewport frame rates.

### 2.3 Heterogeneous Compute Acceleration
- **CUDA & ROCm**: Double-precision (FP64) sparse solvers for FEA and high-speed LBM CFD.
- **Metal & Vulkan**: GPU viewport rendering, GPU color picking, and real-time topology optimization isosurfaces.
- **CPU Parallel Fallback**: Rayon multi-threading with SIMD for exact CAD boolean operations.

### 2.4 Strict Safety Boundaries
- Unsafe driver bindings are isolated exclusively within backend crates (`oxide-backend-*`) and safe FFI wrappers (`oxide-ffi-safe`).
- All application and domain logic is 100% safe Rust.
