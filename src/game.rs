use gamercade_rs::{
    prelude::{self as gc},
    GraphicsParameters,
};
use hecs::{EntityBuilder, World};
use rapier2d::{
    na::Vector2,
    prelude::{ActiveCollisionTypes, ActiveEvents, ColliderBuilder, RigidBodyBuilder},
};

use crate::physics_simulation::PhysicsSimulation;

pub const GRAVITY: f32 = -7.0;
pub const MOVEMENT_SPEED_GROUNDED: f32 = 0.4;
pub const MOVEMENT_SPEED_AIRBORNE: f32 = 0.075;
pub const JUMP_POWER: f32 = 1.5;
pub const PHYSICS_PIXEL_SCALING: f32 = 1024.0;

// Our game state. Edit this as you wish.
pub struct MyGame {
    world: World,
    physics: PhysicsSimulation,
    screen_width: usize,
    screen_height: usize,
    dt: f32,
}

impl crate::Game for MyGame {
    /// Handle all of your initialization logic here.
    fn init() -> Self {
        use crate::components::*;

        // Initialize our working data
        let mut world = World::new();
        let mut physics = PhysicsSimulation::new();
        let player_count = gc::num_players();
        let screen_width = gc::width();
        let screen_height = gc::height();
        let dt = gc::frame_time();
        let half_width = screen_width as f32 / 2.0;
        let ground_plane_y_offset = screen_height as f32 * 0.1;

        // Create the ground
        // We use 0.98 so we can see the ground better
        let ground_collider = ColliderBuilder::cuboid(
            half_width * 0.98 / PHYSICS_PIXEL_SCALING,
            0.25 / PHYSICS_PIXEL_SCALING,
        )
        .translation(Vector2::new(
            half_width / PHYSICS_PIXEL_SCALING,
            ground_plane_y_offset / PHYSICS_PIXEL_SCALING,
        ))
        .build();
        let ground_collider = physics.collider_set.insert(ground_collider);
        let ground_id = world.spawn(
            EntityBuilder::new()
                .add(ground_collider)
                .add(CollisionType::Floor)
                .build(),
        );
        physics
            .collider_set
            .get_mut(ground_collider)
            .unwrap()
            .user_data = u64::from(ground_id.to_bits()) as u128;

        // Generate an entity for each player
        (0..player_count).for_each(|player_id| {
            // Add the colliders/rigid bodies
            let rigid_body = RigidBodyBuilder::kinematic_velocity_based()
                .lock_rotations()
                .build();
            let rigid_body_handle = physics.rigid_body_set.insert(rigid_body);
            let collider =
                ColliderBuilder::cuboid(64.0 / PHYSICS_PIXEL_SCALING, 64.0 / PHYSICS_PIXEL_SCALING)
                    .active_collision_types(
                        ActiveCollisionTypes::default()
                            | ActiveCollisionTypes::KINEMATIC_FIXED
                            | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
                    )
                    .active_events(ActiveEvents::COLLISION_EVENTS)
                    .build();
            let collider_handle = physics.collider_set.insert_with_parent(
                collider,
                rigid_body_handle,
                &mut physics.rigid_body_set,
            );

            let mut player = EntityBuilder::new();
            player
                .add(PlayerId(player_id))
                .add(Controller::default())
                .add(ActorState::default())
                .add(CollisionType::Character)
                .add(rigid_body_handle)
                .add(collider_handle);

            let player_entity = world.spawn(player.build());
            physics
                .collider_set
                .get_mut(collider_handle)
                .unwrap()
                .user_data = u64::from(player_entity.to_bits()) as u128;
        });

        gc::console_log("Game Initialized");
        Self {
            world,
            physics,
            screen_width,
            screen_height,
            dt,
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
        use crate::systems::*;
        let world = &mut self.world;

        input_system(world);
        jump_system(world, &mut self.physics.rigid_body_set);
        movement_system(world, &mut self.physics.rigid_body_set, self.dt);

        // Step produces a list of collision
        // events which we pass onto the collision system
        let collision_events = self.physics.step();
        collision_system(world, collision_events, &mut self.physics.rigid_body_set);

        respawn_system(world, &mut self.physics.rigid_body_set, self.screen_width, self.screen_height);
    }

    /// Handle all of your rendering code here
    fn draw(&self) {
        use crate::systems::*;
        gc::clear_screen(GraphicsParameters::default());

        render_system(
            &self.world,
            &self.physics,
            self.screen_width,
            self.screen_height,
        );
    }
}
