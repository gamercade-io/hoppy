use hecs::World;

use crate::components::{ButtonState, Controller, PlayerId};
use gamercade_rs::prelude as gc;

/// This system takes input from the controller and makes players jump
pub fn jump_system(world: &mut World) {
    world
        .query_mut::<(&Controller, &PlayerId)>()
        .into_iter()
        .for_each(|(_, (controller, player))| {
            let player = player.0;

            // TODO:
            if controller.a == ButtonState::JustPressed {
                gc::console_log(&format!("player {} jumped", player))
            }
        });
}
