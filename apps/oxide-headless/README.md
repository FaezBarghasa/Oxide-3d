# `oxide-headless` — Oxide-3D CLI & Headless CI/CD Runner

Command-line tool for automated batch processing, file format conversions (STEP, DXF, DWG, STL, 3MF, glTF), simulation execution, and test validation without requiring a display server or GPU window.

---

## ⚡ Key Capabilities

- **Automated Geometry Conversion**: Convert between native `.oxd` containers, STEP AP242, DXF/DWG, and triangulated STL/3MF formats.
- **Batch CAE Simulation**: Run FEA linear static stress or LBM CFD jobs from JSON specification files and export VTK / CSV fields.
- **CAM Toolpath Extraction**: Generate CNC G-code toolpaths from B-Rep geometry in batch pipelines.
- **CI / Pipeline Testing**: Deterministic geometry tessellation and Euler-Poincaré verification for automated quality assurance.

---

## 🛠️ Usage

```bash
# Display help and commands
cargo run -p oxide-headless -- --help

# Convert .oxd project file to STEP AP242
cargo run -p oxide-headless -- open model.oxd --export-step model.step

# Run headless simulation case
cargo run -p oxide-headless -- simulate case_spec.json --output ./results
```
