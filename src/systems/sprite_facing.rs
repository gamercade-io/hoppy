use hecs::World;

use crate::components::{Sprite, SpriteKind, Velocity};

pub fn sprite_facing_system(world: &mut World) {
    world
        .query_mut::<(&Velocity, &mut Sprite)>()
        .into_iter()
        .for_each(|(_, (velocity, sprite))| {
            let flip = match velocity.0.x {
                x if x < -f32::EPSILON => true,
                x if x > f32::EPSILON => false,
                _ => return,
            };

            match &mut sprite.kind {
                SpriteKind::Static(gp) => {
                    *gp = gp.flip_x(flip);
                }
                SpriteKind::Animated(animated) => animated.flip_x = flip,
            }
        })
}
