use hecs::World;
use rapier2d::{
    na::Vector2,
    prelude::{RigidBodyHandle, RigidBodySet},
};

use crate::{
    components::{ActorState, ButtonState, Controller, PlayerId},
    game::JUMP_POWER,
};
use gamercade_rs::prelude as gc;

/// This system takes input from the controller and makes players jump
/// Players can only jump if they are grounded.
pub fn jump_system(world: &mut World, rigidbodies: &mut RigidBodySet) {
    world
        .query_mut::<(&Controller, &PlayerId, &mut ActorState, &RigidBodyHandle)>()
        .into_iter()
        .for_each(|(_, (controller, player, state, rigidbody))| {
            let player = player.0;

            // TODO: Give them some vertical velocity
            if controller.a == ButtonState::JustPressed && *state == ActorState::Grounded {
                let rigidbody = rigidbodies.get_mut(*rigidbody).unwrap();
                let new_velocity = rigidbody.linvel() + Vector2::new(0.0, JUMP_POWER);
                rigidbody.set_linvel(new_velocity, true);
                gc::console_log(&format!("player {} jumped", player))
            }
        });
}
