use gamercade_rs::{
    prelude::{self as gc},
    GraphicsParameters,
};
use hecs::{EntityBuilder, World};

use crate::physics_simulation::PhysicsSimulation;

// Our game state. Edit this as you wish.
pub struct MyGame {
    world: World,
    physics: PhysicsSimulation,
}

impl crate::Game for MyGame {
    /// Handle all of your initialization logic here.
    fn init() -> Self {
        use crate::components::*;

        let mut world = World::new();
        let physics = PhysicsSimulation::new();
        let player_count = gc::num_players();

        // TODO: Add Colliders/Rigid bodies etc
        (0..player_count).for_each(|player_id| {
            let mut player = EntityBuilder::new();
            player.add(PlayerId(player_id)).add(Controller::default());

            let _player_entity = world.spawn(player.build());
        });

        gc::console_log("Game Initialized");
        Self { world, physics }
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
        gc::rect(GraphicsParameters::default().color_index(9), 50, 50, 64, 64);
    }
}
