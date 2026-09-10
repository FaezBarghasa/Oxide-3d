//! Oxide-3D Parametric Feature Tree and Dependency Graph.

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};

/// Type of parametric CAD feature in the model history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureKind {
    /// Datum reference plane or coordinate system.
    DatumPlane {
        /// Datum plane identifier name.
        name: String,
    },
    /// 2D sketch constraint plane.
    Sketch {
        /// Sketch identifier name.
        name: String,
    },
    /// Extrude feature.
    Extrude {
        /// Extrusion linear depth distance.
        depth: f64,
    },
    /// Revolve feature.
    Revolve {
        /// Revolution angle in radians.
        angle_rad: f64,
    },
    /// Fillet feature.
    Fillet {
        /// Fillet blending radius.
        radius: f64,
    },
    /// Chamfer feature.
    Chamfer {
        /// Chamfer offset distance.
        distance: f64,
    },
    /// Shell solid hollow feature.
    Shell {
        /// Wall thickness.
        thickness: f64,
    },
    /// Linear/Circular Pattern feature.
    Pattern {
        /// Number of instances.
        count: usize,
        /// Step spacing.
        spacing: f64,
    },
    /// Boolean Union/Cut/Intersect feature.
    Boolean {
        /// Type of boolean (Union, Difference, Intersection).
        operation: String,
    },
}

/// Type of non-destructive modifier in the modifier stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModifierKind {
    // --- Generate Modifiers ---
    /// Array linear/radial repeat modifier.
    Array {
        /// Number of copies.
        count: usize,
        /// Translation offset per instance [x, y, z].
        offset: [f64; 3],
    },
    /// Bevel / Chamfer modifier.
    Bevel {
        /// Width / radius of bevel.
        width: f64,
        /// Number of segments.
        segments: u32,
    },
    /// Boolean mesh/solid modifier.
    Boolean {
        /// Operation type ("Union", "Difference", "Intersect").
        operation: String,
    },
    /// Decimate mesh polygon count modifier.
    Decimate {
        /// Reduction ratio (0.0 - 1.0).
        ratio: f64,
    },
    /// Edge split sharp feature modifier.
    EdgeSplit {
        /// Split angle threshold in radians.
        split_angle: f64,
    },
    /// Mask modifier based on vertex group or armature.
    Mask {
        /// Invert mask flag.
        invert: bool,
    },
    /// Mirror modifier across Cartesian plane.
    Mirror {
        /// Mirror plane normal [nx, ny, nz].
        axis: [f64; 3],
    },
    /// Multiresolution sculpting modifier.
    Multiresolution {
        /// Sculpt subdivision level.
        sculpt_level: u32,
    },
    /// Remesh modifier (Voxel / Quad / Sharp).
    Remesh {
        /// Voxel dimension or target face count.
        voxel_size: f64,
        /// Smooth shading flag.
        smooth_shading: bool,
    },
    /// Screw / Helical revolution modifier.
    Screw {
        /// Screw pitch / offset distance per turn.
        screw_pitch: f64,
        /// Total revolution iterations.
        iterations: u32,
    },
    /// Skin skeletal extrusion modifier.
    Skin {
        /// Branch smoothing factor.
        branch_smoothing: f64,
    },
    /// Solidify / thicken modifier.
    Solidify {
        /// Thickness of wall.
        thickness: f64,
    },
    /// Subdivision Surface modifier (Catmull-Clark).
    Subdivision {
        /// Subdiv levels.
        levels: u32,
    },
    /// Triangulate polygon faces modifier.
    Triangulate {
        /// Quad method (e.g. "Beauty", "Fixed", "Shortest").
        quad_method: String,
    },
    /// Weld duplicate/overlapping vertices within distance.
    Weld {
        /// Distance threshold.
        distance_threshold: f64,
    },
    /// Wireframe edge tube extraction modifier.
    Wireframe {
        /// Thickness of wireframe struts.
        thickness: f64,
    },

    // --- Deform Modifiers ---
    /// Armature skeletal skinning deform.
    Armature {
        /// Armature object identifier.
        armature_id: String,
    },
    /// Cast to sphere / cylinder / cuboid shape.
    Cast {
        /// Cast shape factor.
        factor: f64,
        /// Target shape ("Sphere", "Cylinder", "Cuboid").
        shape: String,
    },
    /// Curve deform following guide path.
    CurveDeform {
        /// Deformation curve axis (X, Y, Z).
        deform_axis: String,
    },
    /// Displace / deform modifier along normals.
    Displace {
        /// Strength factor.
        strength: f64,
        /// Midlevel offset.
        midlevel: f64,
    },
    /// Hook vertex group to empty object.
    Hook {
        /// Falloff radius.
        falloff: f64,
    },
    /// Laplacian smooth deform.
    LaplacianDeform {
        /// Iteration repeat count.
        iterations: u32,
    },
    /// Lattice 3D cage deformation.
    Lattice {
        /// Strength.
        strength: f64,
    },
    /// Shrinkwrap onto target collision surface.
    Shrinkwrap {
        /// Offset from target surface.
        offset: f64,
        /// Projection wrap method ("NearestSurface", "Project", "NearestVertex").
        wrap_method: String,
    },
    /// Simple Deform (Twist, Bend, Taper, Stretch).
    SimpleDeform {
        /// Deform mode ("Twist", "Bend", "Taper", "Stretch").
        mode: String,
        /// Deform angle or factor.
        factor: f64,
    },
    /// Smooth Laplacian / surface fairing.
    Smooth {
        /// Smoothing factor.
        factor: f64,
        /// Iteration repeat count.
        repeat: u32,
    },
    /// Wave animation deformation.
    Wave {
        /// Wave height amplitude.
        height: f64,
        /// Wave width wavelength.
        width: f64,
    },

    // --- Edit & Normals Modifiers ---
    /// Weighted Normal modifier for hard-surface shading.
    WeightedNormal {
        /// Weight factor.
        weight: f64,
        /// Keep sharp edges.
        keep_sharp: bool,
    },
    /// Smooth by Angle modifier.
    SmoothByAngle {
        /// Auto-smooth angle in degrees.
        angle_deg: f64,
    },

    // --- Physics & Simulation Modifiers ---
    /// Cloth physical simulation modifier.
    Cloth {
        /// Structural stiffness.
        stiffness: f64,
        /// Mass density.
        mass_kg: f64,
    },
    /// Collision boundary modifier.
    Collision {
        /// Surface friction coefficient.
        friction: f64,
    },
    /// Soft Body dynamics modifier.
    SoftBody {
        /// Elastic spring friction.
        friction: f64,
    },
}

/// A non-destructive modifier item in the stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierItem {
    /// Name of modifier (e.g. "Subdivision Surface").
    pub name: String,
    /// Modifier parameters.
    pub kind: ModifierKind,
    /// Enabled in viewport.
    pub is_enabled: bool,
    /// Show in edit mode.
    pub show_in_edit: bool,
}

/// Modifier stack container for non-destructive mesh/solid pipelines.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ModifierStack {
    /// Ordered list of modifiers applied from top to bottom.
    pub modifiers: Vec<ModifierItem>,
}

impl ModifierStack {
    /// Create an empty modifier stack.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a modifier to the end of the stack.
    pub fn add_modifier(&mut self, name: impl Into<String>, kind: ModifierKind) {
        self.modifiers.push(ModifierItem {
            name: name.into(),
            kind,
            is_enabled: true,
            show_in_edit: true,
        });
    }

    /// Remove a modifier by index.
    pub fn remove_modifier(&mut self, index: usize) -> Option<ModifierItem> {
        if index < self.modifiers.len() {
            Some(self.modifiers.remove(index))
        } else {
            None
        }
    }
}

/// A node in the parametric feature tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureNode {
    /// Human-readable feature name (e.g. "Extrude.1").
    pub name: String,
    /// Feature parameters and kind.
    pub kind: FeatureKind,
    /// Suppressed/unsuppressed flag.
    pub is_suppressed: bool,
}

/// Dependency relationship edge between features.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FeatureDependency {
    /// Direct geometric reference (e.g. sketch depends on plane).
    Reference,
    /// Upstream topology consumption.
    Consumes,
}

/// Parametric feature history dependency DAG.
#[derive(Debug, Default)]
pub struct FeatureGraph {
    /// Directed graph connecting features in dependency order.
    pub graph: DiGraph<FeatureNode, FeatureDependency>,
}

impl FeatureGraph {
    /// Create a new empty feature tree.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a feature node to the tree.
    pub fn add_feature(&mut self, node: FeatureNode) -> NodeIndex {
        self.graph.add_node(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_graph_and_modifier_stack() {
        let mut graph = FeatureGraph::new();
        let f1 = graph.add_feature(FeatureNode {
            name: "Base Extrude".into(),
            kind: FeatureKind::Extrude { depth: 50.0 },
            is_suppressed: false,
        });
        let f2 = graph.add_feature(FeatureNode {
            name: "Corner Fillet".into(),
            kind: FeatureKind::Fillet { radius: 5.0 },
            is_suppressed: false,
        });
        graph.graph.add_edge(f1, f2, FeatureDependency::Consumes);
        assert_eq!(graph.graph.node_count(), 2);
        assert_eq!(graph.graph.edge_count(), 1);

        let mut stack = ModifierStack::new();
        stack.add_modifier("Subdiv", ModifierKind::Subdivision { levels: 2 });
        stack.add_modifier(
            "Array",
            ModifierKind::Array {
                count: 3,
                offset: [10.0, 0.0, 0.0],
            },
        );
        assert_eq!(stack.modifiers.len(), 2);
    }
}
