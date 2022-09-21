use hecs::World;

use crate::components::{PhysicsVolume, Position};

/// This system keeps players within the bounds of the arena,
/// wrapping them across the X if they go off the screen (on the opposite side)
pub fn bounds_system(world: &mut World, screen_width: usize) {
    world
        .query_mut::<(&mut Position, &PhysicsVolume)>()
        .into_iter()
        .for_each(|(_, (position, phys))| {
            if position.x.value.is_negative() {
                position.x.value += screen_width as i32;
            } else if position.x.value + phys.width as i32 > screen_width as i32 {
                position.x.value -= screen_width as i32;
            }
        });
}
