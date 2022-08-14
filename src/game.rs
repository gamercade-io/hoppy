use gamercade_rs::{
    prelude::{self as gc},
    GraphicsParameters,
};
use hecs::{EntityBuilder, World};
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::physics_simulation::PhysicsSimulation;

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

        // Create the ground
        let ground_collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        physics.collider_set.insert(ground_collider);

        // Generate an entity for each player
        (0..player_count).for_each(|player_id| {
            // Add the colliders/rigid bodies
            // TODO: Would be good to set a position properly for these.
            let rigid_body = RigidBodyBuilder::dynamic().build();
            let rigid_body_handle = physics.rigid_body_set.insert(rigid_body);
            let collider = ColliderBuilder::cuboid(32.0, 32.0).build();
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
            screen_width: gc::width(),
            screen_height: gc::height(),
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
        use crate::systems::*;
        input_system(&mut self.world);
        jump_system(&mut self.world);
        self.physics.step();
    }

    /// Handle all of your rendering code here
    fn draw(&self) {
        gc::clear_screen(GraphicsParameters::default());

        // TODO: Create a render system and draw the players, and floor
        gc::rect(GraphicsParameters::default().color_index(9), 50, 50, 64, 64);
    }
}
