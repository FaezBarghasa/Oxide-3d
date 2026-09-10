# Oxide-3D — Detailed Design Specification

Detailed design specifications for individual subsystems within Oxide-3D.

---

## 1. Domain Modeling & Dual-Topology Core (`oxide-geo` & `oxide-geo-ops`)

### 1.1 B-Rep Topology Hierarchy
- **Vertex (0D)**: Cartesian coordinate `[f64; 3]`.
- **Edge (1D)**: Oriented connection between two vertices bound to a 3D parametric curve (Line, Circle, NURBS).
- **Wire (1D)**: Closed loop of connected edges forming face boundaries and inner hole cutouts.
- **Face (2D)**: Parametric surface (Plane, Cylinder, Sphere, NURBS) trimmed by outer/inner wires.
- **Shell (2D)**: Connected set of faces with watertight manifold validation.
- **Solid (3D)**: Closed outer shell with optional internal cavity shells.

### 1.2 2D Drafting & Snapping Engine
- **Entities**: Line, Polyline, Circle, Arc, Ellipse, Hatch boundary loops, Text.
- **Precision Snapping (`OsnapMode`)**: 11-mode search (`Endpoint`, `Midpoint`, `Center`, `GeometricCenter`, `Quadrant`, `Intersection`, `Perpendicular`, `Tangent`, `Nearest`, `Parallel`).
- **Layers & Annotation**: `CadLayer`, `DimensionEntity` (Linear, Aligned, Radius, Diameter, Angular), `MultiLeader`, and Paper Space Viewports (`MVIEW`).

### 1.3 Kernel Trait (`GeometryKernel`)
```rust
pub trait GeometryKernel: Send + Sync {
    fn boolean(&self, a: SolidKey, b: SolidKey, opts: BooleanOptions) -> KernelResult<SolidKey>;
    fn extrude(&self, profile: FaceKey, direction: [f64; 3], opts: ExtrudeOptions) -> KernelResult<SolidKey>;
    fn fillet(&self, solid: SolidKey, edges: &[EdgeKey], opts: FilletOptions) -> KernelResult<SolidKey>;
}
```

### 1.4 Bivariate Tensor-Product de Boor & Half-Edge Mesh Kernel
- **NURBS Surface Tensor Product**:
  - Exact triangular recurrence evaluating rational B-spline curves along the $v$-direction rows in homogeneous coordinates $[w x, w y, w z, w]$, followed by $u$-direction de Boor reduction on intermediate points.
  - Guarantees $C^\infty$ continuous analytical surface sampling and outward normal vectors across trimmed/untrimmed NURBS patches.
- **Manifold Half-Edge Mesh Kernel (`HalfEdgeMesh`)**:
  - Typed `SlotMap` keys (`HeVertexKey`, `HalfEdgeKey`, `HeFaceKey`).
  - Topological connectivity supporting 1-ring neighbor traversals, edge splits/collapses, and in-place Laplacian mesh smoothing for sculpt and subdivision workflows.

---

## 2. Event-Sourced Document Model (`oxide-core::event_log`)

- **Append-Only Immutable Stream**: Every atomic modeling action is recorded with a time-ordered UUIDv7 `OperationId`.
- **Temporal Cursor Sliding**: The rollback bar and undo/redo adjust the active execution cursor $k \in [0, N]$ without memory reallocations.
- **Multi-Paradigm Payloads**:
  - `CadFeature`: Extrude, Revolve, Fillet, Chamfer, Pattern.
  - `DccModifier`: Subdivision, Mirror, Bevel, Boolean, Geometry Nodes.
  - `AnimationKey`: Time, property path, and controller value.

---

## 3. Procedural Geometry Nodes Engine (`oxide-nodes`)

- **DAG Structure**: Evaluated using topological sort via `petgraph`.
- **Socket Types**:
  - `Float(f64)`
  - `Int(i64)`
  - `Bool(bool)`
  - `Vector([f64; 3])`
  - `String(String)`
- **Dynamic Caching**: Sub-graphs are memoized with content hashing so only modified branches recompute.

---

## 4. Simulation Architecture (`oxide-sim-*`)

| Subsystem | Method | Accelerators | Primary Use Case |
|---|---|---|---|
| `oxide-sim-fea` | Finite Element Method (Tet4, Tet10) | CUDA, ROCm, faer (CPU) | Linear static stress, modal frequencies |
| `oxide-sim-cfd` | Lattice Boltzmann (D3Q19) + FVM | CUDA, ROCm, Vulkan, Metal | Real-time aerodynamics & thermal flow |
| `oxide-sim-topopt` | SIMP Density Method | CUDA, ROCm, Vulkan | Lightweight structural generative design |

---

## 5. Product Lifecycle Management & Persistence (`oxide-plm`, `oxide-persist`)

### 5.1 PLM Data Models
- **`Item`**: Item ID, Part Number, Revision, Lifecycle State (`InWork`, `InReview`, `Released`, `Obsolete`).
- **`BomEntry`**: Hierarchical tree links with quantities and drawing find numbers.
- **Multi-Level Rollup**: Recursive graph traversal calculating aggregated quantities and extended costs.

### 5.2 Native `.oxd` Container Format
- **Manifest**: JSON format specification and metadata.
- **Payload**: MessagePack binary serialization (`rmp-serde`) with Zstandard stream compression (`zstd`).

---

## 6. Computer-Aided Manufacturing (CAM) & Toolpath Generation (`oxide-cam`)

- **2.5D Pocketing Operations**: Multi-pass axial depth stepdowns with bidirectional zigzag rasterization.
- **Postprocessors**: Formatting toolpath points into standardized CNC programs across Fanuc, Haas, GRBL, and Siemens Sinumerik ISO dialects.

---

## 7. Unified 3ds Max & OpenCADStudio Desktop & Web Platform (`apps/oxide-server`, `crates/oxide-ui`)

- **13-Menu DCC Top Bar**: Standardized menu hierarchy (File, Edit, Tools, Group, Views, Create, Modifiers, Animation, Graph Editors, Rendering, Customize, MaxScript, Help).
- **4-Viewport Quad Layout**: Top, Front, Left, and Perspective views with independent coordinate axes, grid, and toggleable maximization.
- **6-Tab Command Panel**: Create (primitives), Modify (modifier stack), Hierarchy, Motion, Display, and Utilities rollouts.
- **OpenCADStudio Command Line**: Multi-line command history, drafting aliases (`LINE`, `CIRCLE`, `BOX`, `CYLINDER`, `PYRAMID`, `EXTRUDE`, `FILLET`, `BOM`, `GCODE`), live cursor coordinates, and drafting status tags.

---

## 8. Visual Examination & Quality Assurance (`playwright-cli`)

- Automated headless and headed browser verification validating UI rendering, responsive layouts, button interactions, command line inputs, and 3D viewport canvas rendering.
- Continuous visual snapshot capture verifying pixel accuracy across drafting and modeling workflows.
