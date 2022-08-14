use gamercade_rs::text::console_log;
use hecs::World;
use rapier2d::{
    na::Vector2,
    prelude::{RigidBodyHandle, RigidBodySet},
};

use crate::{
    components::{ActorState, Controller},
    game::{GRAVITY, MOVEMENT_SPEED_AIRBORNE, MOVEMENT_SPEED_GROUNDED},
};

pub fn movement_system(world: &mut World, physics: &mut RigidBodySet, dt: f32) {
    world
        .query::<(&Controller, &RigidBodyHandle, &ActorState)>()
        .into_iter()
        .for_each(|(_, (controller, handle, state))| {
            if let Some(rigidbody) = physics.get_mut(*handle) {
                let mut current_velocity = rigidbody.linvel().clone();
                let multiplier = match state {
                    ActorState::Grounded => MOVEMENT_SPEED_GROUNDED,
                    ActorState::Airborne => {
                        current_velocity += Vector2::new(0.0, GRAVITY) * dt;
                        MOVEMENT_SPEED_AIRBORNE
                    }
                    ActorState::Dead => 0.0,
                };
                current_velocity += Vector2::new(controller.movement.x * multiplier, 0.0) * dt;
                rigidbody.set_linvel(current_velocity, true);
            } else {
                console_log("movement_system tried to fetch an invalid rigid body handle");
            }
        });
}
