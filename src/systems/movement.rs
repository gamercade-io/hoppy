use hecs::World;

use crate::{
    components::{ActorState, ButtonState, Controller, PhysicsVolume, PhysicsVolumeKind, Velocity},
    game::{JUMP_POWER, MOVEMENT_SPEED_AIRBORNE, MOVEMENT_SPEED_GROUNDED},
};

/// This system takes input from the controller and makes players jump and move as required
/// Players can only jump if they are grounded.
pub fn movement_system(world: &mut World) {
    world
        .query_mut::<(&Controller, &mut Velocity, &mut PhysicsVolume)>()
        .into_iter()
        .for_each(|(_, (controller, velocity, physics))| {
            let x_mul = match physics.kind {
                PhysicsVolumeKind::Actor(ActorState::Grounded) => {
                    if controller.a == ButtonState::JustPressed {
                        physics.kind = PhysicsVolumeKind::Actor(ActorState::Airborne);
                        velocity.0.y -= JUMP_POWER;
                    }
                    MOVEMENT_SPEED_GROUNDED
                }
                PhysicsVolumeKind::Actor(ActorState::Airborne) => MOVEMENT_SPEED_AIRBORNE,
                _ => return,
            };

            velocity.0.x = controller.movement_x * x_mul;
        });
}
