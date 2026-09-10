# `oxide-desktop` — Oxide-3D Native Desktop GUI Application

High-performance native desktop engineering workstation built in modern Rust with the **Iced** Model-View-Update (MVU) framework and hardware-accelerated **wgpu** graphics pipeline.

---

## 🖥️ Overview

`oxide-desktop` provides a non-blocking, multi-threaded CAD/CAE/CAM/DCC desktop workstation interface. It features:
- **CAD CommandManager & Workspace Switching**: Dockable workspace environments for Parametric Modeling, 2D Drafting, FEA/CFD Simulation, CAM Machining, and PLM Management.
- **3ds Max Command Panel**: Right-hand panel with 6 tabs (*Create*, *Modify*, *Hierarchy*, *Motion*, *Display*, *Utilities*) and expandable rollout groups.
- **OpenCADStudio Drafting Command Prompt**: Bottom AutoCAD-style command terminal with command history, prompt auto-completion, and live crosshair coordinate display.
- **wgpu Omniviewport**: Multi-camera viewports supporting PBR shaded surfaces, technical wireframe overlays, GPU-accelerated picking, and dynamic clipping planes.

---

## 🛠️ Usage

```bash
# Run desktop workstation
cargo run -p oxide-desktop

# Launch in Model Context Protocol (MCP) server mode for AI agent pairing
cargo run -p oxide-desktop -- --mcp
```
