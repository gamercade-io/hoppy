use hecs::{Entity, World};
use rapier2d::prelude::{CollisionEvent, RigidBodyHandle, RigidBodySet};

use gamercade_rs::prelude as gc;

use crate::components::{ActorState, CollisionType};

#[derive(Debug)]
pub struct CollisionEventEntry {
    pub event: CollisionEvent,
    pub entity_a: Entity,
    pub entity_b: Entity,
}

/// This system loops through the list of collision events
/// and processes them.
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
                if event.event.started() {
                    handle_character_character_collision(
                        world,
                        event.entity_a,
                        event.entity_b,
                        rigidbodies,
                    )
                }
            }
        }
    })
}

/// Handles character <--> floor collisions
/// Generally this is just setting the ActorState and adjusting velocities.
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

        let mut new_vel = *rigidbody.linvel();
        new_vel.y = 0.0;

        rigidbody.set_linvel(new_vel, true);

        *character_state = ActorState::Grounded;
    } else {
        *character_state = ActorState::Airborne;
    }
}

/// Handles Character <--> Character collisions
/// This checks for a player bonking another, if its y location is above the other and
/// it is travling downward.
fn handle_character_character_collision(
    world: &mut World,
    character_a: Entity,
    character_b: Entity,
    rigid_bodies: &RigidBodySet,
) {
    let a_handle = world.get::<&RigidBodyHandle>(character_a).unwrap();
    let b_handle = world.get::<&RigidBodyHandle>(character_b).unwrap();

    let a_body = rigid_bodies.get(*a_handle).unwrap();
    let b_body = rigid_bodies.get(*b_handle).unwrap();

    let a_pos = a_body.position().translation.y;
    let b_pos = b_body.position().translation.y;

    let a_vel = a_body.linvel().y;
    let b_vel = b_body.linvel().y;

    // A is above B
    if a_pos > b_pos {
        if a_vel.is_sign_negative() {
            *world.get::<&mut ActorState>(character_b).unwrap() = ActorState::Dead;
        }
    } else if b_vel.is_sign_negative() {
        *world.get::<&mut ActorState>(character_a).unwrap() = ActorState::Dead;
    }
}
