use gamercade_rs::prelude::{self as gc, GraphicsParameters};
use hecs::{EntityBuilder, World};

pub const GRAVITY: f32 = 3.0;
pub const MOVEMENT_SPEED_GROUNDED: f32 = 5.0;
pub const MOVEMENT_SPEED_AIRBORNE: f32 = 1.25;
pub const JUMP_POWER: f32 = 25.0;
pub const PLAYER_HEIGHT: u32 = 52;
pub const PLAYER_WIDTH: u32 = 32;

// Our game state. Edit this as you wish.
pub struct MyGame {
    world: World,
    screen_width: usize,
}

impl crate::Game for MyGame {
    /// Handle all of your initialization logic here.
    fn init() -> Self {
        use crate::components::*;

        // Initialize our working data
        let mut world = World::new();
        let player_count = gc::num_players();
        let screen_width = gc::width();
        let screen_height = gc::height();

        let x_offset = 32;
        let y_offset = 12;

        // Generate an entity for each player
        (0..player_count).for_each(|player_id| {
            // Add the colliders/rigid bodies
            let mut player = EntityBuilder::new();
            player
                .add(PlayerId(player_id))
                .add(Controller::default())
                .add(Position {
                    x: PositionValue::new(screen_width as i32 / 2),
                    y: PositionValue::new(screen_height as i32 / 2),
                })
                .add(PhysicsVolume {
                    width: PLAYER_WIDTH,
                    height: PLAYER_HEIGHT,
                    kind: PhysicsVolumeKind::Actor(ActorState::default()),
                })
                .add(Velocity::default())
                .add(Sprite {
                    x_offset,
                    y_offset,
                    kind: SpriteKind::Animated(AnimatedSprite {
                        palette: player_id as u8,
                        sprite_sheet: player_id as u8,
                        sprite: 0,
                        flip_x: false,
                        flip_y: false,
                    }),
                });

            world.spawn(player.build());
        });

        // Add the floor
        {
            let mut floor = EntityBuilder::new();
            floor
                .add(Position {
                    x: PositionValue::new(0),
                    y: PositionValue::new(screen_height as i32 - 10),
                })
                .add(PhysicsVolume {
                    width: screen_width as u32 - 1,
                    height: 10,
                    kind: PhysicsVolumeKind::Solid,
                });
            world.spawn(floor.build());
        }

        gc::console_log("Game Initialized");
        Self {
            world,
            screen_width,
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
        use crate::systems::*;
        let world = &mut self.world;

        input_system(world);
        movement_system(world);
        sprite_facing_system(world);
        physics_system(world);
        actor_actor_collision_system(world);

        respawn_system(world, self.screen_width);
        bounds_system(world, self.screen_width)
    }

    /// Handle all of your rendering code here
    fn draw(&self) {
        use crate::systems::*;

        // Hardcoded black color from the 0th palette
        gc::clear_screen(GraphicsParameters::default().color_index(16));

        render_system(&self.world);
    }
}
