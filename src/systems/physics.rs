use hecs::{Entity, World};

use crate::{
    components::{ActorState, PhysicsVolume, PhysicsVolumeKind, Position, Velocity},
    game::GRAVITY,
};

struct Solid {
    entity: Entity,
    pos_x: i32,
    pos_y: i32,
    width: u32,
    height: u32,
}

struct Collision {
    actor: Entity,
    solid: Entity,
}

/// This system handles most of the physics movements
pub fn physics_system(world: &mut World) {
    // Set up some working structs
    let mut solids = Vec::new();
    let mut collisions = Vec::new();

    // Populate the list of solids
    solids.extend(
        world
            .query_mut::<(&mut PhysicsVolume, &mut Position)>()
            .into_iter()
            .filter_map(|(entity, (phys, pos))| {
                if let PhysicsVolumeKind::Solid = phys.kind {
                    Some(Solid {
                        entity,
                        pos_x: pos.x.value,
                        pos_y: pos.y.value,
                        width: phys.width,
                        height: phys.height,
                    })
                } else {
                    None
                }
            }),
    );

    // Run the velocity adjustment, and store any collisions inside
    // of the collisions vector
    world
        .query_mut::<(&mut PhysicsVolume, &mut Position, &mut Velocity)>()
        .into_iter()
        .for_each(|(entity, (physics, position, velocity))| {
            // Gravity only applies to Airborne things
            if let PhysicsVolumeKind::Actor(ActorState::Airborne) = physics.kind {
                velocity.0.y += GRAVITY;
            }

            // Handle any X/Y Movement
            let (x_result, y_result) = move_direction(position, physics, velocity, &solids);

            if let Some(x_result) = x_result {
                collisions.push(Collision {
                    actor: entity,
                    solid: x_result,
                })
            }

            if let Some(y_result) = y_result {
                collisions.push(Collision {
                    actor: entity,
                    solid: y_result,
                })
            }
        });

    // Handle Actor <--> Solid Collisions here
    collisions.drain(..).for_each(|collision| {
        if let (Ok(actor), Ok(solid)) = (
            world.get::<&Position>(collision.actor),
            world.get::<&Position>(collision.solid),
        ) {
            // Check for grounding collisions
            if actor.y.value < solid.y.value {
                world
                    .get::<&mut PhysicsVolume>(collision.actor)
                    .unwrap()
                    .kind = PhysicsVolumeKind::Actor(ActorState::Grounded);

                world.get::<&mut Velocity>(collision.actor).unwrap().0.y = 0.0;
            }
        }
    });
}

fn move_direction(
    position: &mut Position,
    phys: &PhysicsVolume,
    amount: &Velocity,
    solids: &[Solid],
) -> (Option<Entity>, Option<Entity>) {
    (
        move_direction_x(position, phys, amount.0.x, solids),
        move_direction_y(position, phys, amount.0.y, solids),
    )
}

fn move_direction_x(
    position: &mut Position,
    phys: &PhysicsVolume,
    amount: f32,
    solids: &[Solid],
) -> Option<Entity> {
    position.x.remainder += amount;
    let mut movement = position.x.remainder.round() as i32;

    if movement != 0 {
        position.x.remainder -= movement as f32;

        let sign = movement.signum();

        while movement != 0 {
            // If no collision
            if let Some(collision) =
                check_collision_at(position.x.value + sign, position.y.value, phys, solids)
            {
                return Some(collision);
            } else {
                position.x.value += sign;
                movement -= sign;
            }
        }
    }

    None
}

fn move_direction_y(
    position: &mut Position,
    phys: &PhysicsVolume,
    amount: f32,
    solids: &[Solid],
) -> Option<Entity> {
    position.y.remainder += amount;
    let mut movement = position.y.remainder.round() as i32;

    if movement != 0 {
        position.y.remainder -= movement as f32;

        let sign = movement.signum();

        while movement != 0 {
            // If no collision
            if let Some(collision) =
                check_collision_at(position.x.value, position.y.value + sign, phys, solids)
            {
                return Some(collision);
            } else {
                position.y.value += sign;
                movement -= sign;
            }
        }
    }

    None
}

fn check_collision_at(x: i32, y: i32, phys: &PhysicsVolume, solids: &[Solid]) -> Option<Entity> {
    for solid in solids {
        if x + phys.width as i32 >= solid.pos_x
            && x <= solid.pos_x + solid.width as i32
            && y + phys.height as i32 >= solid.pos_y
            && y <= solid.pos_y + solid.height as i32
        {
            return Some(solid.entity);
        }
    }

    None
}
