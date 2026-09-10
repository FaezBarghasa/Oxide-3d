# Oxide-3D — Test-Driven Development (TDD) & Quality Assurance Guide

Guidelines, standards, verification matrices, and automated testing workflows for Oxide-3D.

---

## 1. Quality & Safety Policy

Oxide-3D enforces zero-tolerance compiler, linter, and architectural rules across all workspace crates:

```toml
[workspace.lints.rust]
unsafe_code = "deny"
unsafe_op_in_unsafe_fn = "deny"
missing_docs = "warn"
missing_debug_implementations = "warn"
rust_2018_idioms = "warn"
rust_2024_compatibility = "warn"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
unwrap_used = "warn"
expect_used = "warn"
panic = "warn"
indexing_slicing = "warn"
todo = "warn"
```

---

## 2. Test Verification Matrix

### 2.1 Kernel Mathematical & Topological Invariants (`crates/oxide-geo`, `crates/oxide-math`)
- **Euler-Poincaré Invariant**: Verify $V - E + F = 2(S - G) + W$ for all closed B-Rep solid topologies.
- **de Boor Numerical Stability**: Verify bivariate NURBS surface points evaluate within $10^{-9}$ tolerance of analytical reference spheres and cylinders.
- **Half-Edge Manifold Consistency**:
  - Exactly two half-edges per manifold edge (`half_edge.twin.twin == half_edge`).
  - Next cycle closure: following `next` around any face returns to start in $n$ steps ($n \ge 3$).
  - 1-ring neighbor Laplacian smoothing preserves manifold boundary vertex positions.

### 2.2 CAM & CNC Toolpath Verification (`crates/oxide-cam`)
- **Z-Slice Stepdown**: Ensure generated toolpaths never exceed specified `max_stepdown` per pass.
- **Stepover Coverage**: Verify consecutive raster passes overlap within $(1 - \text{stepover\_pct}) \times \text{tool\_diameter}$.
- **G-code Dialect Output**:
  - Fanuc: Mandatory `%` header/trailer, line numbers `N...`, feed `F...`.
  - Haas: Work coordinate system `G54`, tool length offset `G43 H...`.
  - GRBL: Compact coordinates without optional line numbering, `M3/M5` spindle control.
  - Siemens: Block structure adhering to Sinumerik standards.

### 2.3 PLM & BOM Cost Rollup Verification (`crates/oxide-plm`)
- **Recursive Cost Rollups**: Verify multi-level tree aggregation accounts for quantity multiplier and scrap percentage compounding:
  $$\text{Unit Cost} = \sum_{\text{sub-items}} \left( \text{Item Cost} \times \text{Quantity} \times (1 + \text{Scrap}) \right)$$
- **Lead Time Invariant**: Assembly lead time cannot be less than the maximum lead time of its subcomponents plus assembly time.

### 2.4 Persistence & Compression Verification (`crates/oxide-persist`)
- **Roundtrip Identity**: `load_document(save_document(doc)) == doc`.
- **Stream Integrity**: Corrupted zstd streams or mismatched schema headers must return clean `Err(PersistError)` without panic.
- **Compression Efficiency**: High-density geometric datasets must demonstrate $\ge 60\%$ compression relative to raw JSON.

### 2.5 End-to-End Headless Visual Testing (`playwright-cli`)
- Verify responsive layout across Desktop (1920x1080), Tablet (1024x768), and Mobile (375x667).
- Verify 3ds Max Command Panel tab switching and rollout toggle state preservation.
- Verify OpenCADStudio command prompt input execution, status bar coordinate updates, and error messaging.
- Verify perspective and quad-viewport WebGL / Canvas rendering without browser console warnings.

---

## 3. Standard Verification Commands

```bash
# 1. Format verification across entire workspace
cargo fmt --check

# 2. Workspace compilation and target verification
cargo check --workspace --all-targets

# 3. Strict Clippy lint check with warnings treated as errors
cargo clippy --workspace --all-targets -- -D warnings

# 4. Comprehensive test execution across all 45 crates
cargo test --workspace

# 5. Targeted crate test execution (examples)
cargo test -p oxide-geo --lib
cargo test -p oxide-cam --lib
cargo test -p oxide-plm --lib
cargo test -p oxide-persist --lib

# 6. Web platform automated visual test via Playwright CLI
cargo run -p oxide-server &
SERVER_PID=$!
sleep 1
playwright-cli open http://127.0.0.1:8080
playwright-cli click "text=Inspect BOM"
playwright-cli fill "input[placeholder*='AutoCAD']" "BOX"
playwright-cli press "Enter"
playwright-cli screenshot --path tests/e2e/quad_viewport_snapshot.png
kill $SERVER_PID
```
