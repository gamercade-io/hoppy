use hecs::World;

use crate::components::{AnimState, Sprite, SpriteKind, Velocity};

pub fn sprite_facing_system(world: &mut World) {
    world
        .query_mut::<(&Velocity, &mut Sprite)>()
        .into_iter()
        .for_each(|(_, (velocity, sprite))| {
            let flip = match velocity.0.x {
                x if x < -f32::EPSILON => Some(true),
                x if x > f32::EPSILON => Some(false),
                _ => None,
            };

            match &mut sprite.kind {
                SpriteKind::Static(gp) => {
                    if let Some(flip) = flip {
                        *gp = gp.flip_x(flip)
                    }
                }
                SpriteKind::Animated(animated) => {
                    let mut next_anim = None;

                    if let Some(flip) = flip {
                        animated.flip_x = flip;

                        next_anim = Some(AnimState::Running);
                    } else {
                        next_anim = Some(AnimState::Idle);
                    }

                    if let Some(next_anim) = next_anim {
                        if next_anim != animated.current_anim {
                            animated.sprite = next_anim.get_range().start;
                            animated.frame_count = 0;
                            animated.current_anim = next_anim;
                        }
                    }
                }
            }
        })
}
