use hecs::World;
use rapier2d::{
    na::Vector2,
    prelude::{RigidBodyHandle, RigidBodySet},
};

use crate::{
    components::{ActorState, PlayerId},
    game::PHYSICS_PIXEL_SCALING,
};
use gamercade_rs::prelude as gc;

pub fn respawn_system(
    world: &mut World,
    rigidbodies: &mut RigidBodySet,
    screen_width: usize,
    screen_height: usize,
) {
    world
        .query_mut::<(&RigidBodyHandle, &mut ActorState, &PlayerId)>()
        .into_iter()
        .for_each(|(_, (handle, state, player))| {
            if ActorState::Dead == *state {
                if let Some(rigidbody) = rigidbodies.get_mut(*handle) {
                    let x = gc::random_float_range(
                        0.0,
                        (screen_width as f32 / 2.0) / PHYSICS_PIXEL_SCALING,
                    );
                    let y = screen_height as f32 / PHYSICS_PIXEL_SCALING;
                    let pos = Vector2::new(x, y);

                    gc::console_log(&format!("respawning player: {} to {}", player.0, pos));

                    rigidbody.set_translation(pos, true);
                    *state = ActorState::Airborne;
                } else {
                    gc::console_log("respawn_system tried to access an invalid RigidBodyHandle")
                }
            };
        });
}
