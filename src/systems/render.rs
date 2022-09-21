use hecs::World;

use crate::components::{PhysicsVolume, Position, Sprite};
use gamercade_rs::prelude::{self as gc, GraphicsParameters};

/// This system renders collision volumes and player sprites (if they have one)
pub fn render_system(world: &World) {
    world
        .query::<(Option<&Sprite>, &Position, Option<&PhysicsVolume>)>()
        .into_iter()
        .for_each(|(_, (sprite, physics, collision))| {
            if let Some(sprite) = sprite {
                // Draw the Sprite
                let gp = sprite.kind.as_parameters();
                let x = physics.x.value - sprite.x_offset as i32;
                let y = physics.y.value - sprite.y_offset as i32;
                gc::sprite(gp, 0, x, y);
            }

            // Draw the Aabb
            if let Some(collision) = collision {
                gc::rect(
                    GraphicsParameters::new(),
                    physics.x.value,
                    physics.y.value,
                    collision.width,
                    collision.height,
                );
            }
        })
}
