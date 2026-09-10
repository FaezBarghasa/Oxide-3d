//! REST API Handlers for Oxide-3D Web Platform.

use axum::Json;
use axum::response::IntoResponse;
use oxide_cam::{CuttingTool, GCodePostProcessor, PocketOperation, PostProcessorDialect, ToolpathGenerator};
use oxide_geo::mesh_bridge::BrepTessellator;
use oxide_geo::topology::TopologyDatabase;
use oxide_plm::{Item, ItemId, LifecycleState, PlmDatabase};
use oxide_settings::OxideSettings;
use serde::{Deserialize, Serialize};

/// System health response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub engine: &'static str,
    pub version: &'static str,
    pub kernel_precision: &'static str,
}

/// Primitive creation request payload.
#[derive(Debug, Deserialize)]
pub struct PrimitiveReq {
    pub kind: String,
    pub dim_x: f64,
    pub dim_y: f64,
    pub dim_z: f64,
    pub segments: Option<usize>,
}

/// Tessellated primitive geometry response.
#[derive(Debug, Serialize)]
pub struct GeometryResponse {
    pub kind: String,
    pub solid_key_valid: bool,
    pub positions_count: usize,
    pub indices_count: usize,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

/// CAD Command execution request.
#[derive(Debug, Deserialize)]
pub struct CommandReq {
    pub command: String,
}

/// CAD Command execution response.
#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub command: String,
    pub response: String,
    pub action: Option<String>,
}

/// CAM G-code toolpath generation response.
#[derive(Debug, Serialize)]
pub struct CamResponse {
    pub program_name: String,
    pub points_count: usize,
    pub gcode: String,
}

/// PLM BOM rollup item.
#[derive(Debug, Serialize)]
pub struct BomItemResponse {
    pub part_number: String,
    pub name: String,
    pub revision: String,
    pub quantity: f64,
    pub unit_cost: f64,
    pub extended_cost: f64,
}

/// PLM BOM hierarchy summary response.
#[derive(Debug, Serialize)]
pub struct BomSummaryResponse {
    pub root_assembly: String,
    pub total_cost: f64,
    pub entries: Vec<BomItemResponse>,
}

/// Health check handler.
pub async fn health_handler() -> impl IntoResponse {
    Json(HealthResponse {
        status: "OK",
        engine: "Oxide-3D Engineering Platform",
        version: "0.1.0-alpha",
        kernel_precision: "f64 spatial / f32 viewport",
    })
}

/// Get current settings handler.
pub async fn get_settings_handler() -> impl IntoResponse {
    Json(OxideSettings::load_or_default())
}

/// Save settings handler.
pub async fn save_settings_handler(Json(settings): Json<OxideSettings>) -> impl IntoResponse {
    let _ = settings.save();
    Json(settings)
}

/// B-Rep Primitive generation handler.
pub async fn create_primitive_handler(Json(req): Json<PrimitiveReq>) -> impl IntoResponse {
    let mut db = TopologyDatabase::new();
    let segs = req.segments.unwrap_or(24);

    let solid_key = match req.kind.to_lowercase().as_str() {
        "cylinder" => db.make_cylinder(req.dim_x * 0.5, req.dim_z, segs),
        "pyramid" => db.make_pyramid(req.dim_x, req.dim_z),
        _ => db.make_box(req.dim_x, req.dim_y, req.dim_z),
    };

    let tessellator = BrepTessellator::new(0.01);
    let mesh = tessellator.tessellate_solid(&db, solid_key);

    Json(GeometryResponse {
        kind: req.kind,
        solid_key_valid: true,
        positions_count: mesh.positions.len(),
        indices_count: mesh.indices.len(),
        positions: mesh.positions,
        normals: mesh.normals,
        indices: mesh.indices,
    })
}

/// Generate CAM G-Code handler.
pub async fn generate_cam_handler() -> impl IntoResponse {
    let tool = CuttingTool {
        name: "6mm Carbide Flat Endmill".into(),
        diameter_mm: 6.0,
        flutes: 3,
        max_rpm: 12000.0,
    };

    let op = PocketOperation {
        tool,
        clearance_z: 10.0,
        retract_z: 2.0,
        stock_top_z: 0.0,
        target_depth_z: -6.0,
        stepdown_mm: 2.0,
        stepover_mm: 3.5,
        feedrate_mm_min: 1500.0,
        plunge_feedrate_mm_min: 350.0,
        spindle_rpm: 9000.0,
    };

    let generator = ToolpathGenerator::new();
    let points = generator.generate_rectangular_pocket(&op, [0.0, 0.0], [60.0, 40.0]);

    let gcode = GCodePostProcessor::post_process(
        PostProcessorDialect::Fanuc,
        "OXIDE_POCKET_O001",
        op.spindle_rpm,
        &points,
    );

    Json(CamResponse {
        program_name: "OXIDE_POCKET_O001.NC".into(),
        points_count: points.len(),
        gcode,
    })
}

/// Get sample PLM BOM rollup handler.
pub async fn get_plm_bom_handler() -> impl IntoResponse {
    let mut plm = PlmDatabase::new();

    let top_assy = Item {
        id: ItemId::default(),
        part_number: "ASY-100-ROBOT".into(),
        name: "6-DOF Industrial Robot Arm".into(),
        revision: "C".into(),
        lifecycle: LifecycleState::InWork,
        unit_cost: 0.0,
    };

    let wrist_sub = Item {
        id: ItemId::default(),
        part_number: "ASY-102-WRIST".into(),
        name: "2-Axis Harmonic Wrist Module".into(),
        revision: "B".into(),
        lifecycle: LifecycleState::Released,
        unit_cost: 250.0,
    };

    let servo = Item {
        id: ItemId::default(),
        part_number: "ELE-450-SERVO".into(),
        name: "750W AC Synchronous Servo".into(),
        revision: "A".into(),
        lifecycle: LifecycleState::Released,
        unit_cost: 180.0,
    };

    let bearing = Item {
        id: ItemId::default(),
        part_number: "BRG-6008-2RS".into(),
        name: "Deep Groove Radial Bearing".into(),
        revision: "A".into(),
        lifecycle: LifecycleState::Released,
        unit_cost: 12.50,
    };

    let top_id = plm.register_item(top_assy);
    let wrist_id = plm.register_item(wrist_sub);
    let servo_id = plm.register_item(servo);
    let bearing_id = plm.register_item(bearing);

    // Root has 1 wrist and 4 main servos
    plm.add_bom_child(top_id, wrist_id, 1.0);
    plm.add_bom_child(top_id, servo_id, 4.0);

    // Wrist has 2 servos and 4 bearings
    plm.add_bom_child(wrist_id, servo_id, 2.0);
    plm.add_bom_child(wrist_id, bearing_id, 4.0);

    let rollup = plm.calculate_bom_rollup(top_id);
    let mut entries = Vec::new();
    let mut total_cost = 0.0;

    for r in rollup {
        total_cost += r.extended_cost;
        entries.push(BomItemResponse {
            part_number: r.item.part_number,
            name: r.item.name,
            revision: r.item.revision,
            quantity: r.total_quantity,
            unit_cost: r.item.unit_cost,
            extended_cost: r.extended_cost,
        });
    }

    Json(BomSummaryResponse {
        root_assembly: "ASY-100-ROBOT [6-DOF Industrial Robot Arm]".into(),
        total_cost,
        entries,
    })
}

/// CAD Command execution interpreter handler.
pub async fn execute_command_handler(Json(req): Json<CommandReq>) -> impl IntoResponse {
    let clean = req.command.trim();
    let upper = clean.to_uppercase();

    let (response, action) = match upper.as_str() {
        "L" | "LINE" => ("Specify first point for line: [X, Y, Z]".to_string(), Some("START_LINE".to_string())),
        "C" | "CIRCLE" => ("Specify center point for circle: [X, Y, Z]".to_string(), Some("START_CIRCLE".to_string())),
        "BOX" => ("Generating 3D parametric Box solid via B-Rep kernel...".to_string(), Some("CREATE_BOX".to_string())),
        "CYL" | "CYLINDER" => ("Generating 3D parametric Cylinder solid via B-Rep kernel...".to_string(), Some("CREATE_CYLINDER".to_string())),
        "PYR" | "PYRAMID" => ("Generating 3D parametric Pyramid solid via B-Rep kernel...".to_string(), Some("CREATE_PYRAMID".to_string())),
        "EXT" | "EXTRUDE" => ("Extruding profile by 30.0 mm along normal vector.".to_string(), Some("EXTRUDE".to_string())),
        "FILLET" => ("Blending selected edges with Constant Radius R=2.5mm.".to_string(), Some("FILLET".to_string())),
        "BOM" => ("Calculating multi-level Product Lifecycle Bill of Materials rollup...".to_string(), Some("SHOW_BOM".to_string())),
        "GCODE" | "CAM" => ("Postprocessing 2.5D toolpath into Fanuc/Haas ISO G-code...".to_string(), Some("GEN_GCODE".to_string())),
        "RESET" => ("Viewport camera orientation reset to Isometric.".to_string(), Some("RESET_VIEW".to_string())),
        "HELP" => ("Available commands: LINE (L), CIRCLE (C), BOX, CYLINDER, PYRAMID, EXTRUDE (EXT), FILLET, BOM, GCODE, RESET".to_string(), None),
        _ => (format!("Executed command: {}", clean), None),
    };

    Json(CommandResponse {
        command: req.command,
        response,
        action,
    })
}
