# Oxide-3D — Application Workflow & Interaction Flow

Step-by-step user interaction, command dispatch, asynchronous compute execution, and data lifecycle pipelines across desktop and web architectures.

---

## 1. Unified User Action to Render Loop (Desktop GUI & Web Platform)

```text
[User Interaction: Click Extrude / Type 'EXT' / Web UI Primitive]
        |
        v
[Front-End Dispatch Layer]
        |-- Desktop: Iced UI Message Loop -> OxideUiMessage::Command(OxideCommand::CreateExtrude)
        |-- Web: OpenCADStudio Command Prompt / 3ds Max Command Panel -> Fetch POST /api/command or /api/geometry/primitive
        |
        v
[CommandBus & Transaction Log] -> Appends to Persistent Event Log (crates/oxide-persist)
        |
        v
[Tokio / Rayon Async Worker Pool]
        |-- Evaluate Feature Graph DAG (crates/oxide-feature)
        |-- Compute Kernel Geometry (crates/oxide-geo, crates/oxide-geo-ops)
        |     - B-Rep Topological Extrusion / CSG Boolean
        |     - Bivariate de Boor Surface Evaluation (NURBS)
        |     - HalfEdge Mesh Refinement & Laplacian Smoothing
        |-- Real-Time Tessellation (BrepTessellator -> Triangles + Normals)
        |
        v
[Display Rendering Subsystem]
        |-- Desktop: wgpu Omniviewport Command Buffer -> PBR WGSL Pipeline (crates/oxide-render)
        |-- Web: Quad Viewport HTML5 Canvas -> Top / Front / Left (Orthographic) + Perspective (3D Wireframe/PBR)
        |
        v
[Interactive Feedback @ 60+ FPS on Screen]
```

---

## 2. 3ds Max & OpenCADStudio Dual UX Interaction Flow

1. **AutoCAD-Style Command Prompt (OpenCADStudio Parity)**:
   - User types an alias in the bottom command prompt (e.g. `L` for Line, `C` for Circle, `BOX` for Solid Box, `EXT` for Extrude, `BOM` for PLM Cost Rollup, `GCODE` for CAM synthesis).
   - The command parser evaluates tokenized input, parameters, and active OSNAP modifiers.
   - Live coordinates `(X, Y, Z)` in the status bar update dynamically with mouse movement across viewports.
   - Command history is retained in the terminal log buffer with color-coded syntax highlights.

2. **Command Panel & Quad Viewports (3ds Max Parity)**:
   - Right-side Command Panel organizes parameters into 6 primary tabs: **Create**, **Modify**, **Hierarchy**, **Motion**, **Display**, and **Utilities**.
   - Collapsible rollouts (e.g. *Object Type*, *Parameters*, *Keyboard Entry*) dynamically reveal context-sensitive options.
   - 4-viewport layout displays synchronised Top (XZ), Front (XY), Left (YZ), and Perspective cameras.
   - Single-click maximize/restore (`⛶`) allows full-screen perspective navigation.

---

## 3. Computer-Aided Manufacturing (CAM) Toolpath Pipeline

```text
[B-Rep Solid / 2D Wire Boundary]
        |
        v
[Tool & Machining Strategy Definition] (crates/oxide-cam)
        |-- Tool Specs: Flat Endmill (e.g., Ø6.0 mm, Flute Length: 25.0 mm)
        |-- Machining Params: Spindle 12,000 RPM, Feed 1,200 mm/min, Plunge 300 mm/min
        |-- Strategy: Pocketing (Stepdown: 1.5 mm, Stepover: 45% = 2.7 mm, Climb/Conventional)
        |
        v
[Toolpath Generator (ToolpathGenerator::generate_pocket)]
        |-- Boundary Offset Polygon Calculation
        |-- Z-Level Slicing & Rasterization (Clearance -> Retract -> Cut -> Stepover)
        |-- Motion Segments: Linear, Circular In/Out, Rapid Traverse
        |
        v
[CNC Postprocessor (PostProcessor::post_process)]
        |-- Target Dialect Selection: Fanuc | Haas | GRBL | Siemens
        |-- G-code Emission: G00/G01/G02/G03 with line numbers, modal groups, coolant, spindle ON
        |
        v
[Delivery to CNC Controller / G-code Viewer]
```

---

## 4. Product Lifecycle Management (PLM) & Hierarchical BOM Flow

1. **BOM Tree Construction**:
   - Assemblies and sub-assemblies are registered with unique `ItemId` keys in `crates/oxide-plm`.
   - Each entry links quantity, scrap factor, unit cost, lead time, and revision state (`Draft` -> `InReview` -> `Released` -> `Obsolete`).
2. **Recursive Cost Rollup (`calculate_bom_cost`)**:
   - Traverses the deep component tree calculating:
     $$\text{Total Cost} = \sum_{\text{children}} \left( \text{Unit Cost} \times \text{Quantity} \times (1 + \text{Scrap Rate}) \right)$$
   - Identifies the critical path lead time (maximum child lead time + assembly stage time).
   - Generates summary statistics: total unit cost, component count, and indented multi-tier hierarchy.
3. **Engineering Change Orders (ECO)**:
   - Version-controlled document references and change authorization workflows ensure trace integrity across revisions.

---

## 5. Persistence & Project Container Storage (`.oxd`)

```text
[Document Snapshot: OxdDocument<T>]
        |
        v
[Serialization Layer] (crates/oxide-persist)
        |-- Metadata: OxdManifest (UUID, Title, Author, Schema Version, Timestamp)
        |-- Binary Encoding: MessagePack (rmp-serde) for high compactness and schema flexibility
        |
        v
[Compression Stream Engine]
        |-- Zstandard (zstd) stream compression (level 3 default)
        |-- CRC32 / Integrity Checksums
        |
        v
[Disk Target: .oxd Unified Project File]
```

- Loading reads the decompressed stream, verifies schema version compatibility, and reconstructs the memory-mapped `TopologyDatabase` and `AssemblyScene`.

---

## 6. Interactive Simulation Workflow (FEA / CFD)

1. **Setup Case**: Assign physical materials (`LinearElasticMaterial`), boundary conditions (fixed faces), and loads (forces/pressures).
2. **Mesh Generation**: Generate volumetric tetrahedral elements (`Tet4`/`Tet10`) in parallel using `rayon`.
3. **Hardware Selection**: `oxide-compute-runtime` evaluates device capabilities:
   - If NVIDIA GPU present $\rightarrow$ Select CUDA backend (`cuSPARSE` / `cuSOLVER`).
   - If AMD GPU present $\rightarrow$ Select ROCm / HIP backend.
   - If Apple Silicon present $\rightarrow$ Select Metal backend.
   - Fallback $\rightarrow$ Select CPU multi-threaded backend (`faer`).
4. **Solve & Stream**: Displacements and von Mises stress fields are streamed directly to the GPU for real-time heatmap shader overlay rendering.
5. **Report & Export**: Export results to VTK, CSV, or PLM engineering report formats.

---

## 7. Headless CI/CD & Automated Playwright QA Flow

```bash
# 1. Automated CAD batch conversion & verification
oxide-headless open bracket.oxd --export-step bracket.step

# 2. Automated nightly FEA stress analysis
oxide-headless simulate bracket_case.json --output results/

# 3. Web UI Headless Verification via Playwright CLI
cargo run -p oxide-server &
playwright-cli open http://127.0.0.1:8080
playwright-cli click "text=Inspect BOM"
playwright-cli fill "input[placeholder*='AutoCAD']" "BOX"
playwright-cli press "Enter"
playwright-cli screenshot --path test_quad_viewport.png
```
