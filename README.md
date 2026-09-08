# Oxide-3D — Industrial CAD/CAE/CAM/PLM Platform

High-performance, GPU-accelerated, event-sourced engineering operating system built in modern Rust (2024 edition).

## Architecture Overview

- **UI Shell (`apps/oxide-desktop`, `crates/oxide-ui`, `crates/oxide-ui-widgets`)**: Non-blocking Iced MVU architecture with dockable workspaces and fuzzy command palette.
- **Rendering (`crates/oxide-render`)**: `wgpu` viewport renderer with PBR, wireframe, technical hidden-line, simulation heatmaps, and GPU picking.
- **Geometry Core (`crates/oxide-geo`, `crates/oxide-geo-ops`, `crates/oxide-geo-io`)**: Exact B-Rep topology database (`slotmap`), NURBS curves/surfaces, parallel booleans, and STEP/STL/OBJ/3MF I/O.
- **Assembly Scene (`crates/oxide-scene`)**: Flat ECS-style scene for 100,000+ parts with BVH spatial indexing.
- **Procedural DAG (`crates/oxide-nodes`, `crates/oxide-feature`)**: Blender-like Geometry Nodes evaluation engine and parametric feature dependency trees.
- **Simulation Fabric (`crates/oxide-sim-core`, `crates/oxide-sim-fea`, `crates/oxide-sim-cfd`, `crates/oxide-sim-topopt`)**: FEA linear static/modal solvers (`faer`), real-time GPU LBM & FVM CFD, and SIMP topology optimization.
- **Mechanics & CAM (`crates/oxide-mech`, `crates/oxide-cam`, `crates/oxide-metrology`)**: Kinematic joints, dynamics (`rapier3d`), 2.5D/3D pocketing, G-code postprocessors, and GD&T evaluation.
- **PLM & Collaboration (`crates/oxide-plm`, `crates/oxide-persist`, `crates/oxide-collab`)**: Item masters, BOM hierarchies, embedded `redb` database, `.oxd` container format, and CRDT multiplayer sync (`automerge`).
- **Automation & Extensibility (`crates/oxide-automation`, `crates/oxide-script-rhai`, `crates/oxide-script-python`, `crates/oxide-plugins-wasm`)**: Unified Command Bus, macro recorder, PyO3 Python scripting, Rhai expression engine, and Wasmtime WASM plugins.

## Building and Running

```bash
# Check formatting and Clippy lints
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings

# Run all workspace unit and integration tests
cargo test --workspace

# Launch the desktop UI application
cargo run -p oxide-desktop

# Launch headless CLI automation
cargo run -p oxide-headless -- --help

# Launch collaboration relay server
cargo run -p oxide-server
```
