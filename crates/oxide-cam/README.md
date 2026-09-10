# `oxide-cam` — Computer-Aided Manufacturing (CAM) & CNC G-Code Generation

Part of the **Oxide-3D** industrial CAD/CAE/CAM/DCC ecosystem. Provides multi-axis toolpath generation, 2.5D pocketing, tool library definitions, and CNC postprocessors.

---

## ⚙️ Core Architecture

- **Toolpath Synthesis (`crates/oxide-cam/src/lib.rs`)**:
  - `Tool`: Defines cutting tool geometry (diameter, flute length, corner radius, spindle speed, cutting feed, plunge feed).
  - `PocketStrategy`: Configures pocket depth, stepdown increment, stepover percentage, climb/conventional milling, and clearance heights.
  - `ToolpathGenerator`: Slices boundary volumes into planar Z-levels and computes rasterized cut passes, linking moves, and rapid traverses.
  - `MotionSegment`: Linear (`G01`), Circular (`G02`/`G03`), and Rapid (`G00`) movements.
- **CNC Postprocessor (`PostProcessor`)**:
  - Emits formatted CNC programs for multiple machine controller dialects:
    - **Fanuc**: Block numbers (`N...`), standard modal safety blocks, spindle and coolant commands.
    - **Haas**: Work offset (`G54`), tool height compensation (`G43 H...`).
    - **GRBL**: Compact coordinate stream optimized for lightweight microcontrollers and makers.
    - **Siemens**: Sinumerik industrial CNC dialect.

---

## 🛠️ Usage Example

```rust
use oxide_cam::{GcodeDialect, PocketStrategy, PostProcessor, Tool, ToolpathGenerator};

// 1. Configure Cutting Tool
let tool = Tool {
    diameter: 6.0,
    flute_length: 25.0,
    corner_radius: 0.0,
    feed_rate: 1200.0,
    plunge_rate: 300.0,
    spindle_speed: 12000.0,
};

// 2. Configure Pocketing Strategy
let strategy = PocketStrategy {
    stepdown: 2.0,
    stepover_pct: 0.45,
    climb: true,
    clearance_z: 5.0,
    retract_z: 2.0,
};

// 3. Generate Toolpath for an 80 x 60 x 10 mm Pocket
let toolpath = ToolpathGenerator::generate_pocket(80.0, 60.0, 10.0, &tool, &strategy);

// 4. Post-Process to Fanuc G-code
let post = PostProcessor::new(GcodeDialect::Fanuc);
let gcode = post.post_process(&toolpath, &tool);
println!("{gcode}");
```
