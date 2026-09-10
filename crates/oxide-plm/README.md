# `oxide-plm` — Product Lifecycle Management (PLM) & Bill of Materials (BOM)

Part of the **Oxide-3D** industrial CAD/CAE/CAM/DCC ecosystem. Provides hierarchical BOM data structures, multi-level recursive cost rollups, inventory unit tracking, revision lifecycle states, and embedded `redb` metadata storage.

---

## ⚙️ Core Architecture

- **Data Models (`crates/oxide-plm/src/lib.rs`)**:
  - `ItemId`: Unique identifier for part or assembly inventory items.
  - `LifecycleState`: `Draft`, `InReview`, `Released`, `Obsolete`.
  - `BomItem`: Represents a component node with part number, name, unit cost ($f64$), quantity ($f64$), scrap rate ($f64$), lead time (days), and nested child items.
  - `BomCostSummary`: Aggregated cost breakdown: total base cost, total scrap cost, total lead time, and sub-item count.
- **Algorithms**:
  - `calculate_bom_cost(&BomItem) -> BomCostSummary`: Computes recursive bottom-up financial aggregation across arbitrary assembly depths.
  - Critical path lead time calculation identifying maximum upstream supply delays.
  - Integration with embedded ACID-compliant `redb` database for revision auditing and Engineering Change Orders (ECO).

---

## 🛠️ Usage Example

```rust
use oxide_plm::{calculate_bom_cost, BomItem, LifecycleState};

let mut assembly = BomItem::new("ASM-001", "CNC Spindle Assembly", 250.0, 1.0);
assembly.lead_time_days = 14;

let mut bearing = BomItem::new("BRG-608", "Ceramic Hybrid Bearing", 15.0, 2.0);
bearing.scrap_rate = 0.05; // 5% scrap margin
bearing.lead_time_days = 7;

let mut shaft = BomItem::new("SHF-101", "Hardened Steel Shaft", 45.0, 1.0);
shaft.lead_time_days = 10;

assembly.children.push(bearing);
assembly.children.push(shaft);

let summary = calculate_bom_cost(&assembly);
println!("Total Assembly Cost: ${:.2}", summary.total_cost);
println!("Critical Path Lead Time: {} days", summary.critical_lead_time_days);
```
