use gamercade_rs::{
    prelude::{self as gc},
    GraphicsParameters,
};
use hecs::{EntityBuilder, World};
use rapier2d::{
    na::Vector2,
    prelude::{ActiveEvents, ColliderBuilder, RigidBodyBuilder},
};

use crate::physics_simulation::PhysicsSimulation;

pub const GRAVITY: f32 = -9.81;
pub const MOVEMENT_SPEED_GROUNDED: f32 = 0.025;
pub const MOVEMENT_SPEED_AIRBORNE: f32 = 0.4;
pub const PHYSICS_PIXEL_SCALING: f32 = 1024.0;

// Our game state. Edit this as you wish.
pub struct MyGame {
    world: World,
    physics: PhysicsSimulation,
    screen_width: usize,
    screen_height: usize,
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
        let half_width = screen_width as f32 / 2.0;
        let ground_plane_y_offset = screen_height as f32 * 0.1;

        // Create the ground
        // We use 0.98 so we can see the ground better
        let ground_collider = ColliderBuilder::cuboid(
            half_width * 0.98 / PHYSICS_PIXEL_SCALING,
            0.1 / PHYSICS_PIXEL_SCALING,
        )
        .translation(Vector2::new(
            half_width / PHYSICS_PIXEL_SCALING,
            ground_plane_y_offset / PHYSICS_PIXEL_SCALING,
        ))
        .build();
        let ground_collider = physics.collider_set.insert(ground_collider);
        world.spawn(EntityBuilder::new().add(ground_collider).build());

        // Generate an entity for each player
        (0..player_count).for_each(|player_id| {
            // Add the colliders/rigid bodies
            let rigid_body = RigidBodyBuilder::dynamic()
                .lock_rotations()
                .translation(Vector2::new(
                    half_width / PHYSICS_PIXEL_SCALING,
                    screen_height as f32 / PHYSICS_PIXEL_SCALING,
                ))
                .build();
            let rigid_body_handle = physics.rigid_body_set.insert(rigid_body);
            let collider =
                ColliderBuilder::cuboid(64.0 / PHYSICS_PIXEL_SCALING, 64.0 / PHYSICS_PIXEL_SCALING)
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
                .add(rigid_body_handle)
                .add(collider_handle);

            let _player_entity = world.spawn(player.build());
        });

        gc::console_log("Game Initialized");
        Self {
            world,
            physics,
            screen_width,
            screen_height,
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
        use crate::systems::*;
        input_system(&mut self.world);
        jump_system(&mut self.world);
        movement_system(&mut self.world, &mut self.physics.rigid_body_set);
        self.physics.step();
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
