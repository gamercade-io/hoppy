use hecs::{Entity, World};
use rapier2d::prelude::{CollisionEvent, RigidBody, RigidBodyHandle, RigidBodySet};

use gamercade_rs::prelude as gc;

use crate::components::{ActorState, CollisionType};

#[derive(Debug)]
pub struct CollisionEventEntry {
    pub event: CollisionEvent,
    pub entity_a: Entity,
    pub entity_b: Entity,
}

pub fn collision_system(
    world: &mut World,
    mut events: Vec<CollisionEventEntry>,
    rigidbodies: &mut RigidBodySet,
) {
    events.drain(..).for_each(|event| {
        let started = event.event.started();
        let a_type = *world.get::<&CollisionType>(event.entity_a).unwrap();
        let b_type = *world.get::<&CollisionType>(event.entity_b).unwrap();

        match (a_type, b_type) {
            (CollisionType::Floor, CollisionType::Floor) => unreachable!(),
            (CollisionType::Floor, CollisionType::Character)
            | (CollisionType::Character, CollisionType::Floor) => {
                if a_type == CollisionType::Character {
                    handle_character_floor_collision(world, event.entity_a, rigidbodies, started);
                } else {
                    handle_character_floor_collision(world, event.entity_b, rigidbodies, started);
                }
            }
            (CollisionType::Character, CollisionType::Character) => {
                gc::console_log("TODO: Charater & Charater collosion response")
            }
        }
    })
}

fn handle_character_floor_collision(
    world: &mut World,
    character: Entity,
    rigidbodies: &mut RigidBodySet,
    started: bool,
) {
    let mut character_state = world.get::<&mut ActorState>(character).unwrap();

    if started {
        let rigidbody = world.get::<&RigidBodyHandle>(character).unwrap();
        let rigidbody = rigidbodies.get_mut(*rigidbody).unwrap();

        let mut new_vel = rigidbody.linvel().clone();
        new_vel.y = 0.0;

        rigidbody.set_linvel(new_vel, true);

        *character_state = ActorState::Grounded;
    } else {
        *character_state = ActorState::Airborne;
    }
}
