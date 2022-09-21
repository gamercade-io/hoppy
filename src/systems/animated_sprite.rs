use hecs::World;

use crate::components::{Sprite, SpriteKind, TICKS_PER_FRAME};

pub fn animated_sprite_system(world: &mut World) {
    world
        .query_mut::<&mut Sprite>()
        .into_iter()
        .for_each(|(_, sprite)| match &mut sprite.kind {
            SpriteKind::Static(_) => (),
            SpriteKind::Animated(animated) => {
                animated.frame_count += 1;

                if animated.frame_count == TICKS_PER_FRAME {
                    animated.sprite += 1;
                    animated.frame_count = 0;

                    if animated.sprite == animated.current_range.end {
                        animated.sprite = animated.current_range.start;
                    }
                }
            }
        })
}
