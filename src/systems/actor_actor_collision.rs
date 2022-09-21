use std::cmp::Ordering;

use hecs::{Entity, World};

use crate::components::{ActorState, PhysicsVolume, PhysicsVolumeKind, Position, Velocity};

struct Collision {
    first: Entity,
    second: Entity,
}

pub fn actor_actor_collision_system(world: &mut World) {
    let mut collisions = Vec::new();

    // Collect all Actor / Actor Overlapping AABBs here
    world
        .query::<(&Position, &PhysicsVolume)>()
        .into_iter()
        .filter(|(_, (_, first_phys))| matches!(first_phys.kind, PhysicsVolumeKind::Actor(..)))
        .for_each(|(first, first_comps)| {
            world
                .query::<(&Position, &PhysicsVolume)>()
                .into_iter()
                .filter(|(second, (_, second_phys))| {
                    first != *second && matches!(second_phys.kind, PhysicsVolumeKind::Actor(..))
                })
                .for_each(|(second, second_comps)| {
                    if aabb_aabb(first_comps, second_comps) {
                        collisions.push(Collision { first, second })
                    }
                })
        });

    collisions.drain(..).for_each(|collision| {
        if let (Ok(first), Ok(second)) = (
            world.get::<&Position>(collision.first),
            world.get::<&Position>(collision.second),
        ) {
            // First is above second
            let result = match first.y.value.cmp(&second.y.value) {
                Ordering::Less => Some((collision.first, collision.second)),
                Ordering::Greater => Some((collision.second, collision.first)),
                Ordering::Equal => None,
            };

            if let Some((top, bottom)) = result {
                if let Ok(true) = world
                    .get::<&Velocity>(top)
                    .map(|velocity| velocity.0.y.is_sign_positive())
                {
                    ko_player(world, bottom)
                }
            }
        }
    });
}

fn ko_player(world: &World, entity: Entity) {
    world.get::<&mut PhysicsVolume>(entity).unwrap().kind =
        PhysicsVolumeKind::Actor(ActorState::Dead);
}

fn aabb_aabb(first: (&Position, &PhysicsVolume), second: (&Position, &PhysicsVolume)) -> bool {
    first.0.x.value <= second.0.x.value + second.1.width as i32
        && first.0.x.value + first.1.width as i32 >= second.0.x.value
        && first.0.y.value <= second.0.y.value + second.1.height as i32
        && first.0.y.value + first.1.height as i32 >= second.0.y.value
}
