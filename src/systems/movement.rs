use gamercade_rs::text::console_log;
use hecs::World;
use rapier2d::{
    na::Vector2,
    prelude::{RigidBodyHandle, RigidBodySet},
};

use crate::{
    components::{ActorState, Controller},
    game::{MOVEMENT_SPEED_AIRBORNE, MOVEMENT_SPEED_GROUNDED},
};

pub fn movement_system(world: &mut World, physics: &mut RigidBodySet) {
    world
        .query::<(&Controller, &RigidBodyHandle, &ActorState)>()
        .into_iter()
        .for_each(|(_, (controller, handle, state))| {
            if let Some(rigidbody) = physics.get_mut(*handle) {
                let multiplier = match state {
                    ActorState::Grounded => MOVEMENT_SPEED_GROUNDED,
                    ActorState::Airborne => MOVEMENT_SPEED_AIRBORNE,
                    ActorState::Dead => 0.0,
                };

                rigidbody.set_linvel(Vector2::new(controller.movement.x * multiplier, 0.0), true);
            } else {
                console_log("movement_system tried to fetch an invalid rigid body handle");
            }
        });
}
