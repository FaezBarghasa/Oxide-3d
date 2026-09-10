//! FeatureManager Design Tree & PropertyManager Panel state models.

use serde::{Deserialize, Serialize};

/// Item type inside the FeatureManager design tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreeItemKind {
    /// Sensors monitor folder.
    Sensors,
    /// Global equations / design table.
    Equations,
    /// Drawing annotations folder.
    Annotations,
    /// Material physical properties assignment.
    Material(String),
    /// Standard reference plane (Front, Top, Right).
    ReferencePlane(String),
    /// World origin point (0, 0, 0).
    Origin,
    /// Solid body instance.
    SolidBody(String),
    /// Surface body sheet.
    SurfaceBody(String),
    /// 2D/3D Parametric Sketch.
    Sketch {
        /// Name of sketch.
        name: String,
        /// Feature suppression state.
        suppressed: bool,
    },
    /// Applied Solid Feature (Extrude, Revolve, Fillet).
    Feature {
        /// Name of feature.
        name: String,
        /// Feature type / category.
        kind: String,
        /// Feature suppression state.
        suppressed: bool,
        /// Has errors or warnings.
        has_error: bool,
    },
    /// Assembly Component Instance.
    Component {
        /// Name of component instance.
        name: String,
        /// Fixed (anchor) vs Free floating.
        is_fixed: bool,
        /// Suppressed state.
        suppressed: bool,
    },
    /// Kinematic Assembly Mate.
    Mate {
        /// Name of mate.
        name: String,
        /// Mate subtype (Coincident, Concentric, Distance).
        mate_type: String,
        /// Suppressed state.
        suppressed: bool,
    },
    /// Sheet metal flat pattern folder.
    FlatPattern,
}

/// Single entry node in the FeatureManager Tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureTreeNode {
    /// Unique node ID.
    pub id: usize,
    /// Display text.
    pub label: String,
    /// Item kind.
    pub kind: TreeItemKind,
    /// Parent node ID if nested.
    pub parent_id: Option<usize>,
    /// Child node IDs.
    pub children: Vec<usize>,
    /// Selected flag.
    pub is_selected: bool,
    /// Expanded in UI tree view.
    pub is_expanded: bool,
}

/// FeatureManager Design Tree Model.
#[derive(Debug, Clone, Default)]
pub struct FeatureManagerTree {
    /// List of all tree nodes indexed by node ID.
    pub nodes: Vec<FeatureTreeNode>,
    /// Active Rollback Bar index (features after rollback are excluded from active evaluation).
    pub rollback_index: Option<usize>,
    /// Filter query text.
    pub filter_query: String,
}

impl FeatureManagerTree {
    /// Create default Part FeatureManager Tree with standard references.
    #[must_use]
    pub fn default_part_tree() -> Self {
        let mut tree = Self::default();

        let root_items = vec![
            ("Sensors", TreeItemKind::Sensors),
            ("Equations", TreeItemKind::Equations),
            ("Annotations", TreeItemKind::Annotations),
            ("Material <6061-T6 Aluminum>", TreeItemKind::Material("6061-T6 Aluminum".into())),
            ("Front Plane", TreeItemKind::ReferencePlane("Front Plane".into())),
            ("Top Plane", TreeItemKind::ReferencePlane("Top Plane".into())),
            ("Right Plane", TreeItemKind::ReferencePlane("Right Plane".into())),
            ("Origin", TreeItemKind::Origin),
            (
                "Sketch1",
                TreeItemKind::Sketch {
                    name: "Sketch1".into(),
                    suppressed: false,
                },
            ),
            (
                "Boss-Extrude1",
                TreeItemKind::Feature {
                    name: "Boss-Extrude1".into(),
                    kind: "Extrude".into(),
                    suppressed: false,
                    has_error: false,
                },
            ),
            (
                "Fillet1",
                TreeItemKind::Feature {
                    name: "Fillet1".into(),
                    kind: "Fillet".into(),
                    suppressed: false,
                    has_error: false,
                },
            ),
        ];

        for (label, kind) in root_items {
            let id = tree.nodes.len();
            tree.nodes.push(FeatureTreeNode {
                id,
                label: label.into(),
                kind,
                parent_id: None,
                children: Vec::new(),
                is_selected: false,
                is_expanded: true,
            });
        }

        tree
    }
}

/// PropertyManager Input Field Type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyField {
    /// Numerical dimension with unit (e.g. "30.0 mm").
    Dimension {
        /// Label name.
        label: String,
        /// Current numeric value.
        value: f64,
        /// Unit string (mm, deg, in).
        unit: String,
    },
    /// Selection bucket (Entities list).
    SelectionBox {
        /// Label name.
        label: String,
        /// Current selections description.
        items: Vec<String>,
        /// Is active for picking.
        is_active: bool,
    },
    /// Boolean toggle option.
    Toggle {
        /// Label.
        label: String,
        /// Value.
        checked: bool,
    },
    /// Dropdown choice.
    Dropdown {
        /// Label.
        label: String,
        /// Options.
        options: Vec<String>,
        /// Selected index.
        selected: usize,
    },
}

/// PropertyManager Panel Model.
#[derive(Debug, Clone)]
pub struct PropertyManagerModel {
    /// Title of the active PropertyManager command.
    pub title: String,
    /// Message instruction banner.
    pub message: String,
    /// Whether PropertyManager is open.
    pub is_active: bool,
    /// Grouped property boxes.
    pub groups: Vec<(String, Vec<PropertyField>)>,
}

impl Default for PropertyManagerModel {
    fn default() -> Self {
        Self {
            title: "Extrude Boss/Base".to_string(),
            message: "Select a sketch or planar face to extrude, or specify parameters below.".to_string(),
            is_active: false,
            groups: vec![
                (
                    "Direction 1".to_string(),
                    vec![
                        PropertyField::Dropdown {
                            label: "End Condition".into(),
                            options: vec![
                                "Blind".into(),
                                "Through All".into(),
                                "Up to Next".into(),
                                "Up to Vertex".into(),
                                "Up to Surface".into(),
                                "Mid Plane".into(),
                            ],
                            selected: 0,
                        },
                        PropertyField::Dimension {
                            label: "Depth (D1)".into(),
                            value: 30.0,
                            unit: "mm".into(),
                        },
                        PropertyField::Toggle {
                            label: "Draft On/Off".into(),
                            checked: false,
                        },
                    ],
                ),
                (
                    "Selected Contours".to_string(),
                    vec![PropertyField::SelectionBox {
                        label: "Selected Contours".into(),
                        items: vec!["Sketch1 (Contour.1)".into()],
                        is_active: true,
                    }],
                ),
            ],
        }
    }
}
