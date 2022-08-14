use std::sync::Arc;

use hecs::Entity;
use parking_lot::Mutex;
use rapier2d::{na::Vector2, prelude::*};

use crate::game::GRAVITY;
use crate::systems::CollisionEventEntry;
use gamercade_rs::prelude as gc;

pub struct PhysicsSimulation {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    gravity: Vector2<f32>,
    physics_pipeline: PhysicsPipeline,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl PhysicsSimulation {
    pub fn new() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            gravity: Vector2::new(0.0, GRAVITY),
            physics_pipeline: PhysicsPipeline::new(),
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::default(),
            broad_phase: BroadPhase::default(),
            narrow_phase: NarrowPhase::default(),
            impulse_joint_set: ImpulseJointSet::default(),
            multibody_joint_set: MultibodyJointSet::default(),
            ccd_solver: CCDSolver::default(),
        }
    }

    pub fn step(&mut self) -> Vec<CollisionEventEntry> {
        let collision_event_handler = CollisionEventHandler::default();

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(),
            &collision_event_handler,
        );

        Arc::try_unwrap(collision_event_handler.events)
            .expect("Physics Step: arc still had refs")
            .into_inner()
    }
}

#[derive(Default)]
pub struct CollisionEventHandler {
    events: Arc<Mutex<Vec<CollisionEventEntry>>>,
}

impl EventHandler for CollisionEventHandler {
    fn handle_collision_event(
        &self,
        _bodies: &RigidBodySet,
        colliders: &ColliderSet,
        event: CollisionEvent,
        _contact_pair: Option<&ContactPair>,
    ) {
        let entity_a = colliders.get(event.collider1()).unwrap().user_data as u64;
        let entity_b = colliders.get(event.collider2()).unwrap().user_data as u64;

        let entity_a = Entity::from_bits(entity_a).unwrap();
        let entity_b = Entity::from_bits(entity_b).unwrap();

        let out = CollisionEventEntry {
            event,
            entity_a,
            entity_b,
        };

        self.events.lock().push(out)
    }

    fn handle_contact_force_event(
        &self,
        _dt: Real,
        _bodies: &RigidBodySet,
        _colliders: &ColliderSet,
        _contact_pair: &ContactPair,
        _total_force_magnitude: Real,
    ) {
        gc::console_log("handle_collision_event");
    }
}
