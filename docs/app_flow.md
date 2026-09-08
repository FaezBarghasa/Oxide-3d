# Oxide-3D — Application Workflow & Interaction Flow

Step-by-step user interaction, command dispatch, and asynchronous compute execution pipelines.

---

## 1. User Action to Render Loop (Desktop GUI)

```text
[User Interaction: Click Extrude]
        |
        v
[Iced UI Event Loop: OxideUiMessage::Command(OxideCommand::CreateExtrude)]
        |
        v
[CommandBus Dispatch] -> Appends to Persistent Operation Log (oxide-persist)
        |
        v
[Tokio Background Task / Rayon Pool]
        |-- Evaluate Feature DAG (oxide-feature)
        |-- Execute Kernel Extrusion (oxide-geo-ops)
        |-- Generate Viewport Triangulation Mesh
        |
        v
[wgpu Command Buffer Encoding] (oxide-render)
        |-- Upload Vertex/Index Buffers
        |-- Dispatch PBR WGSL Pipeline
        |
        v
[Display Frame @ 60+ FPS on Screen]
```

---

## 2. Interactive Simulation Workflow (FEA / CFD)

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

## 3. Headless CI/CD Automation Flow

```bash
# Automated CAD batch conversion & verification
oxide-headless open bracket.oxd --export-step bracket.step

# Automated nightly FEA stress analysis
oxide-headless simulate bracket_case.json --output results/
```
