# Oxide-3D — Test-Driven Development (TDD) & Quality Assurance Guide

Guidelines, standards, and execution commands for maintaining the highest engineering reliability in Oxide-3D.

---

## 1. Quality & Safety Policy

Oxide-3D enforces strict compiler and linter rules across all workspace crates:

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

### 2.1 Unit & Property-Based Tests (`proptest`, `rstest`)
- Mathematical invariants (tolerance checks, orientation predicates).
- Topology invariants (Euler-Poincaré formula $V - E + F = 2$ on closed manifolds).
- Feature DAG cycle prevention.

### 2.2 Golden Geometry File Tests (`insta`)
- Regression tests verifying exact STEP AP242 and STL triangulation binary outputs against reference models.

### 2.3 Analytical FEA & CFD Verification
- Cantilever beam bending against Euler-Bernoulli analytical solutions.
- Lid-driven cavity benchmarks for Lattice Boltzmann fluid solvers.

---

## 3. Standard Verification Commands

```bash
# Verify formatting
cargo fmt --check

# Check compilation of all targets and crates
cargo check --workspace --all-targets

# Run Clippy with zero-tolerance for warnings
cargo clippy --workspace --all-targets -- -D warnings

# Run all tests in parallel
cargo test --workspace

# Check dependencies, security advisories, and licenses
cargo deny check
```
