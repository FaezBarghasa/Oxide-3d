# Oxide-3D — AI Assistant Context & Developer Cheatsheet

High-density technical reference for AI assistants, contributors, and pair programmers working on Oxide-3D.

---

## 1. Monorepo Map

| Crate Path | Role & Key Crates / Types |
|---|---|
| `crates/oxide-core` | `EntityKey`, `OperationId`, `OxideCommand`, `OxideEvent`, `CommandBus` |
| `crates/oxide-math` | `Interval`, `orient2d`, `orient3d`, `Tolerance`, `Transform3` |
| `crates/oxide-geo` | `TopologyDatabase`, `Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid` |
| `crates/oxide-geo-ops` | `GeometryKernel` trait, `BooleanOptions`, `ExtrudeOptions`, `FilletOptions` |
| `crates/oxide-geo-io` | `TriangleMesh`, `export_stl_binary`, STEP / 3MF / glTF adapters |
| `crates/oxide-feature` | `FeatureGraph` (petgraph DAG), `FeatureNode`, `FeatureKind` |
| `crates/oxide-nodes` | `OxideNode` trait, `NodeSocketValue`, `SocketDef` |
| `crates/oxide-scene` | `AssemblyScene`, `SceneEntity`, `DisplayMode` (ECS style) |
| `crates/oxide-render` | `Camera`, `RenderPipelineManager`, WGSL shaders |
| `crates/oxide-hal` | `AcceleratorBackend` trait, `BackendCapabilities`, `DeviceBuffer`, `KernelArg` |
| `crates/oxide-compute-runtime` | `ComputeRuntime`, `ComputeJob`, `KernelRegistry` |
| `crates/oxide-backend-*` | `CpuBackend`, `WgpuBackend`, `CudaBackend`, `RocmBackend`, `VulkanBackend`, `MetalBackend`, `Dx12Backend`, `OpenClBackend` |
| `crates/oxide-sim-*` | `SimulationMesh`, `LinearElasticMaterial`, `solve_linear_static`, `LbmSolver` |
| `crates/oxide-plm` | `Item`, `ItemId`, `LifecycleState`, `BomEntry` (embedded `redb`) |
| `crates/oxide-persist` | `OxdManifest`, `.oxd` package container storage |
| `crates/oxide-ui` | `OxideApp`, `WorkspaceMode`, `OxideUiMessage` (Iced MVU) |
| `apps/oxide-desktop` | GUI Desktop Application entry point |
| `apps/oxide-headless` | CLI automation / batch processing runner |
| `apps/oxide-server` | Collaboration sync and compute relay server |

---

## 2. Critical Development Rules

1. **Unsafe Code Isolation**: `unsafe` is strictly forbidden in domain and application crates (`unsafe_code = "deny"`). Driver FFI must reside in `crates/oxide-backend-*` or `crates/oxide-ffi-safe`.
2. **Asynchronous Separation**: Never block Tokio async worker threads with heavy mathematical or meshing operations. Always offload to Rayon or `spawn_blocking`.
3. **Immutability & Replay**: Prefer append-only event-sourced logging (`OxideCommand`) over fragile in-place document mutations.
4. **Tolerance Awareness**: Use `Tolerance::points_equal` and exact geometric predicates (`orient2d`, `orient3d`) instead of direct raw float equality `==`.
