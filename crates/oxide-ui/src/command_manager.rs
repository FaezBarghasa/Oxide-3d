//! SolidWorks-style CommandManager tabs, contextual toolbars, and action catalog.

use serde::{Deserialize, Serialize};

/// Document context for CommandManager tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DocumentContext {
    /// Single Solid/Surface Part.
    #[default]
    Part,
    /// Multi-Component Assembly.
    Assembly,
    /// 2D Multi-Sheet Technical Drawing.
    Drawing,
}

/// CommandManager Tab categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommandTab {
    /// 3D Features (Extrude, Revolve, Sweep, Loft, Fillet, Pattern).
    Features,
    /// 2D & 3D Parametric Sketching.
    Sketch,
    /// Measurement, Diagnostics, SimulationXpress.
    Evaluate,
    /// Model-Based Definition, GD&T & Tolerancing.
    DimXpert,
    /// Sheet Metal Flanges, Bends, Flat Pattern.
    SheetMetal,
    /// Weldment Profiles, Gussets, End Caps, Cut Lists.
    Weldments,
    /// Mold Parting Lines, Shut-off Surfaces, Core/Cavity.
    MoldTools,
    /// Advanced Freeform Surfaces.
    Surfaces,
    /// Assembly Mates, Exploded Views, Interference Check.
    Assembly,
    /// Assembly Layout Sketches & Blocks.
    Layout,
    /// Drawing View Layout (Projected, Section, Detail).
    ViewLayout,
    /// Drawing Annotations, Notes, BOM Tables, Balloons.
    Annotation,
    /// Office Add-Ins (Motion, Simulation, Routing, Toolbox).
    OfficeProducts,
}

impl CommandTab {
    /// Return human-readable label for the tab.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Features => "Features",
            Self::Sketch => "Sketch",
            Self::Evaluate => "Evaluate",
            Self::DimXpert => "DimXpert",
            Self::SheetMetal => "Sheet Metal",
            Self::Weldments => "Weldments",
            Self::MoldTools => "Mold Tools",
            Self::Surfaces => "Surfaces",
            Self::Assembly => "Assembly",
            Self::Layout => "Layout",
            Self::ViewLayout => "View Layout",
            Self::Annotation => "Annotation",
            Self::OfficeProducts => "Office Products",
        }
    }
}

/// Individual tool item inside a CommandManager Tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandToolDef {
    /// Display name of the tool.
    pub name: String,
    /// Icon or mnemonic identifier.
    pub icon_id: &'static str,
    /// Action identifier for command dispatching.
    pub action_id: &'static str,
    /// Tooltip explanation.
    pub tooltip: String,
}

impl CommandToolDef {
    /// Create a new CommandToolDef.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        icon_id: &'static str,
        action_id: &'static str,
        tooltip: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            icon_id,
            action_id,
            tooltip: tooltip.into(),
        }
    }
}

/// CommandManager State Model.
#[derive(Debug, Clone)]
pub struct CommandManagerModel {
    /// Current document context.
    pub doc_context: DocumentContext,
    /// Active tab in the CommandManager.
    pub active_tab: CommandTab,
    /// Whether CommandManager is floating or docked.
    pub is_floating: bool,
}

impl Default for CommandManagerModel {
    fn default() -> Self {
        Self {
            doc_context: DocumentContext::Part,
            active_tab: CommandTab::Features,
            is_floating: false,
        }
    }
}

impl CommandManagerModel {
    /// Create a new CommandManagerModel.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return available tabs for current document context.
    #[must_use]
    pub fn get_available_tabs(&self) -> Vec<CommandTab> {
        match self.doc_context {
            DocumentContext::Part => vec![
                CommandTab::Features,
                CommandTab::Sketch,
                CommandTab::Evaluate,
                CommandTab::DimXpert,
                CommandTab::Surfaces,
                CommandTab::SheetMetal,
                CommandTab::Weldments,
                CommandTab::MoldTools,
                CommandTab::OfficeProducts,
            ],
            DocumentContext::Assembly => vec![
                CommandTab::Assembly,
                CommandTab::Layout,
                CommandTab::Sketch,
                CommandTab::Evaluate,
                CommandTab::OfficeProducts,
            ],
            DocumentContext::Drawing => vec![
                CommandTab::ViewLayout,
                CommandTab::Annotation,
                CommandTab::Sketch,
                CommandTab::Evaluate,
                CommandTab::DimXpert,
            ],
        }
    }

    /// Return list of tools in the specified tab.
    #[must_use]
    pub fn get_tools(&self, tab: CommandTab) -> Vec<CommandToolDef> {
        match tab {
            CommandTab::Features => vec![
                CommandToolDef::new(
                    "Extruded Boss/Base",
                    "extrude",
                    "cmd.extrude",
                    "Extrudes a sketch profile into a 3D solid",
                ),
                CommandToolDef::new(
                    "Revolved Boss/Base",
                    "revolve",
                    "cmd.revolve",
                    "Revolves a sketch profile around a centerline axis",
                ),
                CommandToolDef::new(
                    "Swept Boss/Base",
                    "sweep",
                    "cmd.sweep",
                    "Sweeps a profile along a guide path trajectory",
                ),
                CommandToolDef::new(
                    "Lofted Boss/Base",
                    "loft",
                    "cmd.loft",
                    "Blends between two or more cross-section profiles",
                ),
                CommandToolDef::new(
                    "Boundary Boss/Base",
                    "boundary",
                    "cmd.boundary",
                    "Creates high-precision boundary blend volume",
                ),
                CommandToolDef::new(
                    "Extruded Cut",
                    "cut_extrude",
                    "cmd.cut_extrude",
                    "Removes material by extruding a profile",
                ),
                CommandToolDef::new(
                    "Revolved Cut",
                    "cut_revolve",
                    "cmd.cut_revolve",
                    "Removes material by revolving a profile",
                ),
                CommandToolDef::new(
                    "Hole Wizard",
                    "hole_wizard",
                    "cmd.hole_wizard",
                    "Inserts standardized ISO/ANSI countersunk/tapped holes",
                ),
                CommandToolDef::new(
                    "Fillet",
                    "fillet",
                    "cmd.fillet",
                    "Creates rounded edges or variable radius blends",
                ),
                CommandToolDef::new(
                    "Chamfer",
                    "chamfer",
                    "cmd.chamfer",
                    "Bevels selected edges with angle-distance parameters",
                ),
                CommandToolDef::new(
                    "Rib",
                    "rib",
                    "cmd.rib",
                    "Creates thin-walled structural reinforcing webs",
                ),
                CommandToolDef::new(
                    "Draft",
                    "draft",
                    "cmd.draft",
                    "Applies taper angles to mold parting faces",
                ),
                CommandToolDef::new(
                    "Shell",
                    "shell",
                    "cmd.shell",
                    "Hollows out a solid body leaving specified wall thickness",
                ),
                CommandToolDef::new(
                    "Mirror",
                    "mirror",
                    "cmd.mirror",
                    "Mirrors features or bodies across a plane",
                ),
                CommandToolDef::new(
                    "Linear Pattern",
                    "lin_pat",
                    "cmd.lin_pattern",
                    "Replicates features along direction vectors",
                ),
                CommandToolDef::new(
                    "Circular Pattern",
                    "circ_pat",
                    "cmd.circ_pattern",
                    "Replicates features radially around an axis",
                ),
                CommandToolDef::new(
                    "Reference Geometry",
                    "ref_geom",
                    "cmd.ref_geom",
                    "Creates Planes, Axes, Coordinate Systems, Points",
                ),
            ],
            CommandTab::Sketch => vec![
                CommandToolDef::new(
                    "Line",
                    "line",
                    "sketch.line",
                    "Draws line segments and continuous polylines",
                ),
                CommandToolDef::new(
                    "Circle",
                    "circle",
                    "sketch.circle",
                    "Draws center-radius or 3-point perimeter circles",
                ),
                CommandToolDef::new(
                    "Rectangle",
                    "rect",
                    "sketch.rect",
                    "Draws corner, center, or 3-point aligned rectangles",
                ),
                CommandToolDef::new(
                    "Arc",
                    "arc",
                    "sketch.arc",
                    "Draws 3-point, centerpoint, or tangent arcs",
                ),
                CommandToolDef::new(
                    "Spline",
                    "spline",
                    "sketch.spline",
                    "Draws smooth B-splines with curvature control points",
                ),
                CommandToolDef::new(
                    "Polygon",
                    "polygon",
                    "sketch.polygon",
                    "Draws equilateral polygons (N-gons)",
                ),
                CommandToolDef::new(
                    "Smart Dimension",
                    "smart_dim",
                    "sketch.smart_dim",
                    "Applies parametric distance, angle, and radius dimensions",
                ),
                CommandToolDef::new(
                    "Sketch Fillet",
                    "sk_fillet",
                    "sketch.fillet",
                    "Rounds corners in 2D sketch contours",
                ),
                CommandToolDef::new(
                    "Offset Entities",
                    "sk_offset",
                    "sketch.offset",
                    "Offsets contours by specified normal clearance",
                ),
                CommandToolDef::new(
                    "Convert Entities",
                    "sk_convert",
                    "sketch.convert",
                    "Projects model edges onto active sketch plane",
                ),
                CommandToolDef::new(
                    "Trim Entities",
                    "sk_trim",
                    "sketch.trim",
                    "Power trims or corner cuts intersecting sketch curves",
                ),
                CommandToolDef::new(
                    "Mirror Entities",
                    "sk_mirror",
                    "sketch.mirror",
                    "Mirrors 2D entities about a centerline",
                ),
                CommandToolDef::new(
                    "Display/Delete Relations",
                    "sk_relations",
                    "sketch.relations",
                    "Inspects and sets Geometric Constraints (Coincident, Tangent, Concentric)",
                ),
            ],
            CommandTab::Evaluate => vec![
                CommandToolDef::new(
                    "Measure",
                    "measure",
                    "eval.measure",
                    "Calculates distances, angles, radius, area between entities",
                ),
                CommandToolDef::new(
                    "Mass Properties",
                    "mass_props",
                    "eval.mass_props",
                    "Computes Volume, Center of Mass, Moments of Inertia",
                ),
                CommandToolDef::new(
                    "Section Properties",
                    "sec_props",
                    "eval.sec_props",
                    "Calculates planar area centroid and moment of inertia",
                ),
                CommandToolDef::new(
                    "Interference Detection",
                    "interference",
                    "eval.interference",
                    "Identifies intersecting solid volumes in assemblies",
                ),
                CommandToolDef::new(
                    "Check Geometry",
                    "check_geom",
                    "eval.check",
                    "Validates B-Rep topological integrity and manifold correctness",
                ),
                CommandToolDef::new(
                    "Import Diagnostics",
                    "import_diag",
                    "eval.import_diag",
                    "Repairs faulty faces and gaps in STEP/IGES bodies",
                ),
                CommandToolDef::new(
                    "Performance Evaluation",
                    "perf_eval",
                    "eval.perf",
                    "Profiles feature rebuild times and scene bottlenecks",
                ),
                CommandToolDef::new(
                    "SimulationXpress",
                    "sim_xpress",
                    "eval.sim_xpress",
                    "Performs rapid linear FEA stress & displacement analysis",
                ),
                CommandToolDef::new(
                    "DFMXpress",
                    "dfm_xpress",
                    "eval.dfm_xpress",
                    "Validates part manufacturability for CNC milling and turning",
                ),
                CommandToolDef::new(
                    "Equations",
                    "equations",
                    "eval.equations",
                    "Manages global mathematical dimensions and relations",
                ),
            ],
            CommandTab::DimXpert => vec![
                CommandToolDef::new(
                    "Auto Dimension Scheme",
                    "dim_auto",
                    "dim.auto_scheme",
                    "Applies full geometric dimensioning and tolerancing scheme",
                ),
                CommandToolDef::new(
                    "Size Dimension",
                    "dim_size",
                    "dim.size",
                    "Applies toleranced diameter, width, or slot dimensions",
                ),
                CommandToolDef::new(
                    "Location Dimension",
                    "dim_loc",
                    "dim.location",
                    "Applies linear position dimension relative to datums",
                ),
                CommandToolDef::new(
                    "Geometric Tolerance",
                    "dim_gdt",
                    "dim.gdt",
                    "Defines ASME Y14.5 / ISO GD&T symbols (Position, Flatness, Runout)",
                ),
                CommandToolDef::new(
                    "Datum",
                    "dim_datum",
                    "dim.datum",
                    "Establishes primary/secondary/tertiary datum reference frames",
                ),
                CommandToolDef::new(
                    "Show Tolerance Status",
                    "dim_status",
                    "dim.status",
                    "Color codes under-constrained and fully constrained faces",
                ),
            ],
            CommandTab::SheetMetal => vec![
                CommandToolDef::new(
                    "Base Flange/Tab",
                    "sm_base",
                    "sm.base_flange",
                    "Creates initial sheet metal plate and sets gauge table",
                ),
                CommandToolDef::new(
                    "Edge Flange",
                    "sm_edge",
                    "sm.edge_flange",
                    "Adds folded flanges along perimeter sheet edges",
                ),
                CommandToolDef::new(
                    "Miter Flange",
                    "sm_miter",
                    "sm.miter_flange",
                    "Creates series of continuous mitered flanges",
                ),
                CommandToolDef::new(
                    "Hem",
                    "sm_hem",
                    "sm.hem",
                    "Adds rolled or closed hems to sheet borders",
                ),
                CommandToolDef::new(
                    "Flatten",
                    "sm_flatten",
                    "sm.flatten",
                    "Unfolds folded sheet metal part into flat fabrication pattern",
                ),
                CommandToolDef::new(
                    "Rip",
                    "sm_rip",
                    "sm.rip",
                    "Creates seams in hollow bodies for sheet metal conversion",
                ),
            ],
            CommandTab::Weldments => vec![
                CommandToolDef::new(
                    "Structural Member",
                    "wm_member",
                    "wm.member",
                    "Extrudes standard beam/pipe profiles along 3D sketch paths",
                ),
                CommandToolDef::new(
                    "End Cap",
                    "wm_endcap",
                    "wm.end_cap",
                    "Caps open ends of hollow structural tube members",
                ),
                CommandToolDef::new(
                    "Gusset",
                    "wm_gusset",
                    "wm.gusset",
                    "Adds triangular or polygonal gussets between intersecting beams",
                ),
                CommandToolDef::new(
                    "Weld Bead",
                    "wm_bead",
                    "wm.weld_bead",
                    "Adds solid weld fillet beads between adjacent bodies",
                ),
                CommandToolDef::new(
                    "Cut List",
                    "wm_cutlist",
                    "wm.cut_list",
                    "Generates cut list table with member lengths and miters",
                ),
            ],
            CommandTab::MoldTools => vec![
                CommandToolDef::new(
                    "Draft Analysis",
                    "mold_draft",
                    "mold.draft_analysis",
                    "Colors faces based on pull direction draft angles",
                ),
                CommandToolDef::new(
                    "Parting Lines",
                    "mold_parting",
                    "mold.parting_lines",
                    "Calculates optimal parting seam curve around core/cavity",
                ),
                CommandToolDef::new(
                    "Shut-off Surfaces",
                    "mold_shutoff",
                    "mold.shutoff_surf",
                    "Patches internal through-holes in molded parts",
                ),
                CommandToolDef::new(
                    "Parting Surfaces",
                    "mold_parting_srf",
                    "mold.parting_surf",
                    "Extends outer parting boundary for tooling split",
                ),
                CommandToolDef::new(
                    "Tooling Split",
                    "mold_split",
                    "mold.tooling_split",
                    "Separates raw stock into Core and Cavity tooling blocks",
                ),
            ],
            CommandTab::Surfaces => vec![
                CommandToolDef::new(
                    "Extruded Surface",
                    "surf_extrude",
                    "surf.extrude",
                    "Extrudes open curve into zero-thickness surface sheet",
                ),
                CommandToolDef::new(
                    "Revolved Surface",
                    "surf_revolve",
                    "surf.revolve",
                    "Revolves open curve around an axis",
                ),
                CommandToolDef::new(
                    "Swept Surface",
                    "surf_sweep",
                    "surf.sweep",
                    "Sweeps open curve along guide trajectory",
                ),
                CommandToolDef::new(
                    "Lofted Surface",
                    "surf_loft",
                    "surf.loft",
                    "Creates smooth freeform surface through cross sections",
                ),
                CommandToolDef::new(
                    "Boundary Surface",
                    "surf_boundary",
                    "surf.boundary",
                    "Creates high-precision tangent/curvature continuous patch",
                ),
                CommandToolDef::new(
                    "Filled Surface",
                    "surf_fill",
                    "surf.fill",
                    "Constructs surface patch bounded by arbitrary closed loop",
                ),
                CommandToolDef::new(
                    "Knit Surface",
                    "surf_knit",
                    "surf.knit",
                    "Stitches adjacent surface sheets into watertight solid",
                ),
                CommandToolDef::new(
                    "Trim Surface",
                    "surf_trim",
                    "surf.trim",
                    "Trims surface sheet using cutting curve or tool",
                ),
                CommandToolDef::new(
                    "Thicken",
                    "surf_thicken",
                    "surf.thicken",
                    "Thickens surface into 3D solid body",
                ),
            ],
            CommandTab::Assembly => vec![
                CommandToolDef::new(
                    "Insert Components",
                    "asm_insert",
                    "asm.insert",
                    "Places external part/sub-assembly files into scene",
                ),
                CommandToolDef::new(
                    "Mate",
                    "asm_mate",
                    "asm.mate",
                    "Applies kinematic mating constraints (Coincident, Concentric, Distance)",
                ),
                CommandToolDef::new(
                    "Exploded View",
                    "asm_explode",
                    "asm.explode",
                    "Creates step-by-step disassembly animations and diagrams",
                ),
                CommandToolDef::new(
                    "Linear Component Pattern",
                    "asm_lin_pat",
                    "asm.lin_pat",
                    "Arrays component instances in linear matrix",
                ),
                CommandToolDef::new(
                    "Circular Component Pattern",
                    "asm_circ_pat",
                    "asm.circ_pat",
                    "Arrays component instances radially",
                ),
                CommandToolDef::new(
                    "Interference Detection",
                    "asm_interfere",
                    "asm.interference",
                    "Checks assembly for overlapping solid volumes",
                ),
                CommandToolDef::new(
                    "Clearance Verification",
                    "asm_clearance",
                    "asm.clearance",
                    "Ensures minimum mechanical clearance between parts",
                ),
            ],
            CommandTab::Layout => vec![
                CommandToolDef::new(
                    "Layout Sketch",
                    "lyt_sketch",
                    "layout.sketch",
                    "Creates master top-down parametric layout sketch",
                ),
                CommandToolDef::new(
                    "Create Block",
                    "lyt_make_block",
                    "layout.make_block",
                    "Groups sketch entities into rigid 2D mechanism block",
                ),
                CommandToolDef::new(
                    "Insert Block",
                    "lyt_inst_block",
                    "layout.insert_block",
                    "Instantiates standard 2D linkage or block",
                ),
            ],
            CommandTab::ViewLayout => vec![
                CommandToolDef::new(
                    "Model View",
                    "drw_model_view",
                    "drw.model_view",
                    "Places orthographic 3D part projection on sheet",
                ),
                CommandToolDef::new(
                    "Projected View",
                    "drw_projected",
                    "drw.projected",
                    "Projects folded view from existing view",
                ),
                CommandToolDef::new(
                    "Section View",
                    "drw_section",
                    "drw.section",
                    "Creates cutaway section view with automatic cross-hatching",
                ),
                CommandToolDef::new(
                    "Detail View",
                    "drw_detail",
                    "drw.detail",
                    "Generates magnified detail callout circle",
                ),
                CommandToolDef::new(
                    "Broken-out Section",
                    "drw_broken_out",
                    "drw.broken_out",
                    "Removes local surface layer to reveal interior cavity",
                ),
            ],
            CommandTab::Annotation => vec![
                CommandToolDef::new(
                    "Smart Dimension",
                    "drw_smart_dim",
                    "annot.smart_dim",
                    "Applies ASME/ISO technical drawing dimensions",
                ),
                CommandToolDef::new(
                    "Note",
                    "drw_note",
                    "annot.note",
                    "Inserts text notes and leader callouts",
                ),
                CommandToolDef::new(
                    "Auto Balloon",
                    "drw_auto_balloon",
                    "annot.auto_balloon",
                    "Automatically adds item number balloons for Bill of Materials",
                ),
                CommandToolDef::new(
                    "Bill of Materials (BOM)",
                    "drw_bom",
                    "annot.bom",
                    "Generates linked multi-level BOM table",
                ),
                CommandToolDef::new(
                    "Center Mark",
                    "drw_center_mark",
                    "annot.center_mark",
                    "Draws standard cross center marks on circular holes",
                ),
                CommandToolDef::new(
                    "Centerline",
                    "drw_centerline",
                    "annot.centerline",
                    "Draws symmetry centerlines on revolved bodies",
                ),
                CommandToolDef::new(
                    "Surface Finish Symbol",
                    "drw_surf_finish",
                    "annot.surf_finish",
                    "Inserts roughness / Ra surface texture callout",
                ),
                CommandToolDef::new(
                    "Weld Symbol",
                    "drw_weld_sym",
                    "annot.weld_symbol",
                    "Inserts AWS/ISO welding specification symbols",
                ),
            ],
            CommandTab::OfficeProducts => vec![
                CommandToolDef::new(
                    "Oxide Motion",
                    "ofc_motion",
                    "office.motion",
                    "Rigid-body kinematic & dynamic solver (Rapier3D)",
                ),
                CommandToolDef::new(
                    "Oxide Simulation (FEA)",
                    "ofc_fea",
                    "office.fea",
                    "Finite Element linear static and modal stress analysis",
                ),
                CommandToolDef::new(
                    "Oxide Flow (CFD)",
                    "ofc_cfd",
                    "office.cfd",
                    "Lattice-Boltzmann Computational Fluid Dynamics simulation",
                ),
                CommandToolDef::new(
                    "Oxide CAM",
                    "ofc_cam",
                    "office.cam",
                    "2.5D to 5-axis CNC toolpath generation and G-code export",
                ),
                CommandToolDef::new(
                    "Oxide Toolbox",
                    "ofc_toolbox",
                    "office.toolbox",
                    "Library of standard fasteners, bearings, gears (ISO/ANSI/DIN)",
                ),
                CommandToolDef::new(
                    "Oxide Photorealistic Render",
                    "ofc_pbr",
                    "office.pbr",
                    "Raymarched & path-traced viewport visualization",
                ),
            ],
        }
    }
}
