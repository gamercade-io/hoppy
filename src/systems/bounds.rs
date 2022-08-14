use hecs::World;
use rapier2d::{
    na::Vector2,
    prelude::{RigidBodyHandle, RigidBodySet},
};

use gamercade_rs::prelude as gc;

use crate::game::PHYSICS_PIXEL_SCALING;

/// This system keeps players within the bounds of the arena,
/// wrapping them across the X if they go off the screen (on the opposite side)
pub fn bounds_system(world: &World, rigidbodies: &mut RigidBodySet, screen_width: usize) {
    world
        .query::<&RigidBodyHandle>()
        .iter()
        .for_each(|(_, handle)| {
            if let Some(rigidbody) = rigidbodies.get_mut(*handle) {
                let pos = rigidbody.position().translation;

                let max_x = screen_width as f32 / PHYSICS_PIXEL_SCALING;

                if pos.x.is_sign_negative() {
                    rigidbody.set_translation(Vector2::new(max_x, pos.y), true)
                } else if pos.x > max_x {
                    rigidbody.set_translation(Vector2::new(0.0, pos.y), true)
                }
            } else {
                gc::console_log("bounds_system tried to fetch an invalid RigidBodyHandle")
            }
        });
}
