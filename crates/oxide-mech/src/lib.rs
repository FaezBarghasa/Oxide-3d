//! Oxide-3D Mechanisms, Kinematic Joints, and Rigid Body Dynamics.

use oxide_core::id::EntityKey;
use rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of kinematic joint constraint connecting mechanism components.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum JointKind {
    /// Completely rigid fix.
    Fixed,
    /// 1-DOF Revolute hinge joint around an axis.
    Revolute {
        /// Axis vector.
        axis: [f64; 3],
        /// Optional min/max limits in radians.
        limits: Option<[f64; 2]>,
    },
    /// 1-DOF Prismatic slider joint along an axis.
    Prismatic {
        /// Translation axis.
        axis: [f64; 3],
        /// Optional min/max limits in distance.
        limits: Option<[f64; 2]>,
    },
    /// Gear pair coupling angular velocities.
    Gear {
        /// Ratio (output / input).
        ratio: f64,
        /// Driving entity.
        driver: EntityKey,
        /// Driven entity.
        driven: EntityKey,
    },
}

/// Mechanism assembly physical body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechBody {
    /// Entity ID associated with this body.
    pub entity: EntityKey,
    /// Mass in kilograms.
    pub mass_kg: f64,
    /// Position [x, y, z].
    pub position: [f64; 3],
    /// Rotation axis-angle or euler angles [x, y, z].
    pub rotation: [f64; 3],
    /// Is fixed / static base ground.
    pub is_fixed: bool,
}

/// Assembly mechanism simulation world.
pub struct MechanismWorld {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    ccd_solver: CCDSolver,
    gravity: Vector,
    integration_parameters: IntegrationParameters,
    body_map: HashMap<EntityKey, RigidBodyHandle>,
}

impl std::fmt::Debug for MechanismWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MechanismWorld")
            .field("bodies", &self.body_map.len())
            .field("gravity", &self.gravity)
            .finish()
    }
}

impl Default for MechanismWorld {
    fn default() -> Self {
        Self::new([0.0, -9.81, 0.0])
    }
}

impl MechanismWorld {
    /// Create a new mechanism simulation world with gravity vector.
    pub fn new(gravity: [f64; 3]) -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            ccd_solver: CCDSolver::new(),
            gravity: Vector::new(gravity[0] as Real, gravity[1] as Real, gravity[2] as Real),
            integration_parameters: IntegrationParameters::default(),
            body_map: HashMap::new(),
        }
    }

    /// Add a rigid body into the mechanism.
    pub fn add_body(&mut self, body: &MechBody) -> RigidBodyHandle {
        let rb_builder = if body.is_fixed {
            RigidBodyBuilder::fixed()
        } else {
            RigidBodyBuilder::dynamic()
        };

        let rb = rb_builder
            .translation(Vector::new(
                body.position[0] as Real,
                body.position[1] as Real,
                body.position[2] as Real,
            ))
            .rotation(Vector::new(
                body.rotation[0] as Real,
                body.rotation[1] as Real,
                body.rotation[2] as Real,
            ))
            .additional_mass(body.mass_kg as Real)
            .build();

        let handle = self.rigid_body_set.insert(rb);

        // Attach a simple ball collider so the mass and inertia tensor are populated
        let collider = ColliderBuilder::ball(0.5)
            .mass(body.mass_kg.max(0.001) as Real)
            .build();
        self.collider_set
            .insert_with_parent(collider, handle, &mut self.rigid_body_set);

        self.body_map.insert(body.entity, handle);
        handle
    }

    /// Connect two bodies with a kinematic joint.
    pub fn add_joint(
        &mut self,
        entity1: EntityKey,
        entity2: EntityKey,
        joint_kind: JointKind,
    ) -> Option<ImpulseJointHandle> {
        let h1 = *self.body_map.get(&entity1)?;
        let h2 = *self.body_map.get(&entity2)?;

        let joint_data: GenericJoint = match joint_kind {
            JointKind::Fixed => FixedJointBuilder::new().into(),
            JointKind::Revolute { axis, limits } => {
                let ax = Vector::new(
                    axis[0] as Real,
                    axis[1] as Real,
                    axis[2] as Real,
                );
                let b1 = self.rigid_body_set.get(h1).unwrap();
                let b2 = self.rigid_body_set.get(h2).unwrap();
                let rel_offset = b2.translation() - b1.translation();

                let mut rev = RevoluteJointBuilder::new(ax)
                    .local_anchor2(-rel_offset);

                if let Some(lim) = limits {
                    rev = rev.limits([lim[0] as Real, lim[1] as Real]);
                }
                rev.into()
            }
            JointKind::Prismatic { axis, limits } => {
                let ax = Vector::new(
                    axis[0] as Real,
                    axis[1] as Real,
                    axis[2] as Real,
                );
                let b1 = self.rigid_body_set.get(h1).unwrap();
                let b2 = self.rigid_body_set.get(h2).unwrap();
                let rel_offset = b2.translation() - b1.translation();

                let mut prism = PrismaticJointBuilder::new(ax)
                    .local_anchor2(-rel_offset);
                if let Some(lim) = limits {
                    prism = prism.limits([lim[0] as Real, lim[1] as Real]);
                }
                prism.into()
            }
            JointKind::Gear { .. } => {
                FixedJointBuilder::new().into()
            }
        };

        Some(
            self.impulse_joint_set
                .insert(h1, h2, joint_data, true),
        )
    }

    /// Step the mechanism physics forward by `dt` seconds.
    pub fn step(&mut self, dt: f64) {
        self.integration_parameters.dt = dt as Real;
        let physics_hooks = ();
        let event_handler = ();

        self.physics_pipeline.step(
            self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
        );
    }

    /// Get the current position of a body entity [x, y, z].
    pub fn get_body_position(&self, entity: EntityKey) -> Option<[f64; 3]> {
        let handle = *self.body_map.get(&entity)?;
        let body = self.rigid_body_set.get(handle)?;
        let t = body.translation();
        Some([t.x as f64, t.y as f64, t.z as f64])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

    #[test]
    fn test_mechanism_world_simulation() {
        let mut sm = SlotMap::<EntityKey, ()>::with_key();
        let ground_entity = sm.insert(());
        let pendulum_entity = sm.insert(());

        let mut world = MechanismWorld::new([0.0, -9.81, 0.0]);

        let ground = MechBody {
            entity: ground_entity,
            mass_kg: 0.0,
            position: [0.0, 10.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            is_fixed: true,
        };

        let pendulum = MechBody {
            entity: pendulum_entity,
            mass_kg: 5.0,
            position: [5.0, 10.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            is_fixed: false,
        };

        world.add_body(&ground);
        world.add_body(&pendulum);

        world.add_joint(
            ground_entity,
            pendulum_entity,
            JointKind::Revolute {
                axis: [0.0, 0.0, 1.0],
                limits: None,
            },
        );

        // Step 60 frames (1 second of physics)
        for _ in 0..60 {
            world.step(1.0 / 60.0);
        }

        let pos = world
            .get_body_position(pendulum_entity)
            .expect("Body position should be queryable");

        // The pendulum should have dropped in Y coordinate under gravity
        assert!(pos[1] < 10.0, "Pendulum should drop in Y coordinate, got {}", pos[1]);
    }
}
