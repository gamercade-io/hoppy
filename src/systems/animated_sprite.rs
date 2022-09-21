use hecs::World;

use crate::components::{Sprite, SpriteKind, TICKS_PER_FRAME};

pub fn animated_sprite_system(world: &mut World) {
    world
        .query_mut::<&mut Sprite>()
        .into_iter()
        .for_each(|(_, sprite)| match &mut sprite.kind {
            SpriteKind::Static(_) => (),
            SpriteKind::Animated(animated) => {
                // Update and tick animations
                animated.frame_count += 1;

                if animated.frame_count == TICKS_PER_FRAME {
                    animated.sprite += 1;
                    animated.frame_count = 0;

                    let range = animated.current_anim.get_range();

                    if animated.sprite == range.end {
                        animated.sprite = range.start;
                    }
                }
            }
        })
}
