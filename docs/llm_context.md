# Oxide-3D — AI Assistant Context & Developer Cheatsheet

High-density technical reference for AI assistants, contributors, and pair programmers working on Oxide-3D.

---

## 1. Monorepo Map (45 Crates + 3 Apps)

| Crate Path | Role & Key Structs / Functions |
|---|---|
| `apps/oxide-desktop` | Iced MVU desktop application shell, non-blocking render loop, CommandManager. |
| `apps/oxide-server` | Axum Web platform, REST API (`/api/*`), unified 3ds Max & OpenCADStudio Web UI. |
| `apps/oxide-headless` | CLI batch runner, format converter, headless test runner. |
| `crates/oxide-core` | `EntityKey`, `OperationId`, `OxideCommand`, `OxideEvent`, `CommandBus`. |
| `crates/oxide-math` | `Interval`, `orient2d`, `orient3d`, `Tolerance`, `Transform3`, SIMD vector math. |
| `crates/oxide-geo` | `TopologyDatabase`, `Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid`, `NurbsSurface` (bivariate de Boor), `HalfEdgeMesh`. |
| `crates/oxide-geo-ops` | `GeometryKernel` trait, `BooleanOptions`, `ExtrudeOptions`, `FilletOptions`, `BrepTessellator`. |
| `crates/oxide-geo-io` | `TriangleMesh`, `export_stl_binary`, STEP AP242, DXF/DWG, 3MF, glTF serializers. |
| `crates/oxide-feature` | `FeatureGraph` (petgraph DAG), `FeatureNode`, `FeatureKind`, procedural rollback. |
| `crates/oxide-nodes` | `OxideNode` trait, `NodeSocketValue`, `SocketDef`, Geometry Nodes graph. |
| `crates/oxide-scene` | `AssemblyScene`, `SceneEntity`, `DisplayMode`, BVH acceleration tree. |
| `crates/oxide-render` | `Camera`, `RenderPipelineManager`, WGSL PBR / technical line shaders. |
| `crates/oxide-backend-wgpu` | Cross-platform GPU graphics & compute backend via `wgpu`. |
| `crates/oxide-hal` | `AcceleratorBackend` trait, `BackendCapabilities`, `DeviceBuffer`, `KernelArg`. |
| `crates/oxide-compute-runtime` | `ComputeRuntime`, `ComputeJob`, `KernelRegistry`. |
| `crates/oxide-backend-*` | `CpuBackend`, `WgpuBackend`, `CudaBackend`, `RocmBackend`, `VulkanBackend`, `MetalBackend`, `Dx12Backend`, `OpenClBackend`. |
| `crates/oxide-sim-core` | Simulation topology, finite element mesh definition (`Tet4`/`Tet10`), boundary conditions. |
| `crates/oxide-sim-fea` | `LinearElasticMaterial`, `solve_linear_static`, stiffness matrix assembly. |
| `crates/oxide-sim-cfd` | 3D Lattice Boltzmann Method (`LbmSolver`, D3Q19, BGK/SRT collision). |
| `crates/oxide-sim-topopt` | SIMP topology optimization, compliance minimization, Helmholtz filtering. |
| `crates/oxide-cam` | 2.5D pocketing (`PocketStrategy`, `ToolpathGenerator`), G-code postprocessing (`PostProcessor`, Fanuc/Haas/GRBL/Siemens). |
| `crates/oxide-plm` | Hierarchical BOM (`BomItem`), recursive cost rollups (`calculate_bom_cost`), lifecycle states, `redb` storage. |
| `crates/oxide-persist` | Project container storage (`OxdManifest`, `OxdDocument<T>`), MessagePack + `zstd` stream compression. |
| `crates/oxide-settings` | Persistent user configuration (`OxideSettings`), TOML serialization, platform directory resolution. |
| `crates/oxide-ui` | Top 13-menu bar, 6-tab Command Panel, 4-viewport quad layout, animation timeline. |
| `crates/oxide-ui-widgets` | Canvas views, rollouts, curve editors, numerical scrubbers, OSNAP indicators. |
| `crates/oxide-automation` | JSON-RPC 2.0 Model Context Protocol (MCP) server for external AI agents. |
| `crates/oxide-script-python` | PyO3 Python embedded runtime and CAD script bindings. |
| `crates/oxide-script-rhai` | Lightweight Rhai embedded scripting engine. |
| `crates/oxide-plugins-wasm` | Sandboxed WebAssembly plugin runtime (`wasmtime`). |
| `crates/oxide-collab` | CRDT multiplayer document synchronization and operational transform. |
| `crates/oxide-mech` | Kinematic joint constraints, degrees of freedom analysis, rigid body physics. |
| `crates/oxide-metrology` | GD&T geometric tolerancing, CMM inspection paths, deviation heatmaps. |

---

## 2. Critical Development Invariants

1. **Safety & Zero Warnings**:
   - `unsafe` is forbidden in all domain and logic crates (`unsafe_code = "deny"`).
   - No `unwrap()` or `expect()` in production code. Explicit `Result<T, E>` or `Option<T>` with defensive error propagation.
2. **Precision Duality**:
   - **$f64$**: All geometry kernel, B-Rep topology, NURBS equations, toolpath computations, and FEA/CFD solvers MUST use 64-bit precision ($f64$) to eliminate numerical drift.
   - **$f32$**: Rendering buffers, GPU vertex streams, and camera matrix transformations use 32-bit floats ($f32$) for bandwidth efficiency.
3. **Asynchronous Separation**:
   - Never block Tokio worker threads with geometry evaluations or Rayon tasks. Offload to background workers.
4. **Tolerance Awareness**:
   - Never compare floats with `==`. Always use `Tolerance::points_equal` and exact robust geometric predicates (`orient2d`, `orient3d`).

---

## 3. UI Command Prompt Aliases (OpenCADStudio Parity)

| Alias | Full Command | Functionality |
|---|---|---|
| `L` | `LINE` | Interactive 2D/3D line creation with OSNAP snaps. |
| `PL` | `PLINE` | Continuous polyline with arc and straight segments. |
| `C` | `CIRCLE` | Center-radius or 3-point circle. |
| `A` | `ARC` | 3-point or start-center-end arc drafting. |
| `REC` | `RECTANGLE` | 2-corner rectangle wire. |
| `BOX` | `BOX` | Parametric B-Rep solid box primitive. |
| `CYL` | `CYLINDER` | Parametric B-Rep solid cylinder primitive. |
| `PYR` | `PYRAMID` | Parametric B-Rep solid pyramid primitive. |
| `EXT` | `EXTRUDE` | Extrude 2D closed wire into 3D B-Rep solid. |
| `REV` | `REVOLVE` | Revolve 2D wire around axis. |
| `FIL` | `FILLET` | Apply rolling-ball edge fillet blend. |
| `CHAM`| `CHAMFER` | Bevel planar boundary edges. |
| `BOM` | `BOM` | Inspect multi-level PLM BOM hierarchy and compute cost rollups. |
| `GCODE`| `CAM_GCODE` | Generate 2.5D pocketing toolpaths and emit G-code. |

---

## 4. 3ds Max Command Panel Organization

1. **Create Tab**: Geometry (Standard Primitives: Box, Sphere, Cylinder, Pyramid; Extended Primitives), Shapes (Lines, Splines), Lights, Cameras.
2. **Modify Tab**: Modifier Stack (Subdivision, Bevel, Extrude, Mirror, Displace), rollout parameter spinners.
3. **Hierarchy Tab**: Pivot point adjustment, forward/inverse kinematics (FK/IK) joint limits.
4. **Motion Tab**: Keyframe controllers, Trajectories, PRS (Position/Rotation/Scale) parameter tracks.
5. **Display Tab**: Viewport display settings (Geometry, Wireframe, Bounding Box, Shaded), frozen states.
6. **Utilities Tab**: Measure tools, MassFX physics configuration, CAM Toolpath synthesizer, PLM Cost Auditor.

---

## 5. Playwright Visual Verification Reference

```bash
# Start oxide-server
cargo run -p oxide-server

# Open browser session on the web UI
playwright-cli open http://127.0.0.1:8080

# Execute interactive UI actions
playwright-cli click "text=Inspect BOM"
playwright-cli fill "input[placeholder*='AutoCAD']" "BOX"
playwright-cli press "Enter"
playwright-cli click "text=+ Cyl"
playwright-cli click "text=CAM G-Code"
playwright-cli click "text=⛶ Toggle Maximize"

# Capture visual state screenshot
playwright-cli screenshot
```
