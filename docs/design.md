# Oxide-3D — Detailed Design Specification

Detailed design specifications for individual subsystems within Oxide-3D.

---

## 1. Domain Modeling & B-Rep Topology (`oxide-geo` & `oxide-geo-ops`)

### 1.1 Topology Hierarchy
- **Vertex (0D)**: Cartesian coordinate `[f64; 3]`.
- **Edge (1D)**: Oriented connection between two vertices bound to a 3D parametric curve (Line, Circle, NURBS).
- **Wire (1D)**: Closed loop of connected edges forming face boundaries and inner hole cutouts.
- **Face (2D)**: Parametric surface (Plane, Cylinder, Sphere, NURBS) trimmed by outer/inner wires.
- **Shell (2D)**: Connected set of faces with watertight manifold validation.
- **Solid (3D)**: Closed outer shell with optional internal cavity shells.

### 1.2 Kernel Trait (`GeometryKernel`)
```rust
pub trait GeometryKernel: Send + Sync {
    fn boolean(&self, a: SolidKey, b: SolidKey, opts: BooleanOptions) -> KernelResult<SolidKey>;
    fn extrude(&self, profile: FaceKey, direction: [f64; 3], opts: ExtrudeOptions) -> KernelResult<SolidKey>;
    fn fillet(&self, solid: SolidKey, edges: &[EdgeKey], opts: FilletOptions) -> KernelResult<SolidKey>;
}
```

---

## 2. Procedural Geometry Nodes Engine (`oxide-nodes`)

- **DAG Structure**: Evaluated using topological sort via `petgraph`.
- **Socket Types**:
  - `Float(f64)`
  - `Int(i64)`
  - `Bool(bool)`
  - `Vector([f64; 3])`
  - `String(String)`
- **Dynamic Caching**: Sub-graphs are memoized with content hashing so only modified branches recompute.

---

## 3. Simulation Architecture (`oxide-sim-*`)

| Subsystem | Method | Accelerators | Primary Use Case |
|---|---|---|---|
| `oxide-sim-fea` | Finite Element Method (Tet4, Tet10) | CUDA, ROCm, faer (CPU) | Linear static stress, modal frequencies |
| `oxide-sim-cfd` | Lattice Boltzmann (D3Q19) + FVM | CUDA, ROCm, Vulkan, Metal | Real-time aerodynamics & thermal flow |
| `oxide-sim-topopt` | SIMP Density Method | CUDA, ROCm, Vulkan | Lightweight structural generative design |

---

## 4. Product Lifecycle Management & Persistence (`oxide-plm`, `oxide-persist`)

### 4.1 PLM Data Models
- **`Item`**: Item ID, Part Number, Revision, Lifecycle State (`InWork`, `InReview`, `Released`, `Obsolete`).
- **`BomEntry`**: Hierarchical tree links with quantities and drawing find numbers.
- **Database Engine**: Embedded zero-overhead transactional `redb`.

### 4.2 Native `.oxd` Container Format
- **Manifest**: JSON format specification and metadata.
- **Operation Log**: Append-only log of `OxideCommand` actions.
- **Chunked Blobs**: Zstd-compressed binary geometry and simulation fields with `memmap2` streaming.
