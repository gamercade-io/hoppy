use hecs::World;

use crate::components::{ActorState, PhysicsVolume, PhysicsVolumeKind, Position, PositionValue};
use gamercade_rs::prelude as gc;

/// This system is responsible for respawning players.
/// Players are ready for respawning when their ActorState is Dead.
pub fn respawn_system(world: &mut World, screen_width: usize) {
    world
        .query_mut::<(&mut PhysicsVolume, &mut Position)>()
        .into_iter()
        .for_each(|(_, (physics, position))| {
            if let PhysicsVolumeKind::Actor(ActorState::Dead) = physics.kind {
                let new_x = gc::random_int_range(0, screen_width as i32);
                let new_y = 0;

                position.x = PositionValue::new(new_x);
                position.y = PositionValue::new(new_y as i32);

                physics.kind = PhysicsVolumeKind::Actor(ActorState::Airborne);
            };
        });
}
