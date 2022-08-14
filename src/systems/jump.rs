use hecs::World;

use crate::components::{ActorState, ButtonState, Controller, PlayerId};
use gamercade_rs::prelude as gc;

/// This system takes input from the controller and makes players jump
pub fn jump_system(world: &mut World) {
    world
        .query_mut::<(&Controller, &PlayerId, &mut ActorState)>()
        .into_iter()
        .for_each(|(_, (controller, player, state))| {
            let player = player.0;

            // TODO: Give them some vertical velocity
            if controller.a == ButtonState::JustPressed && *state == ActorState::Grounded {
                *state = ActorState::Airborne;
                gc::console_log(&format!("player {} jumped", player))
            }
        });
}
