use hecs::World;

use crate::components::{ButtonState, Controller, PlayerId};
use gamercade_rs::prelude as gc;

/// This system takes input from the console state and places it into the "Controller"
/// component to be used throughout the game later
pub fn input_system(world: &mut World) {
    world
        .query_mut::<(&mut Controller, &PlayerId)>()
        .into_iter()
        .for_each(|(_, (controller, player))| {
            let player = player.0;

            // Handle Up/Down
            if gc::button_up_held(player) == Some(true) {
                controller.movement.y = -1.0;
            } else if gc::button_down_held(player) == Some(true) {
                controller.movement.y = 1.0;
            } else {
                controller.movement.y = 0.0;
            }

            // Handle Left/Right
            if gc::button_left_held(player) == Some(true) {
                controller.movement.x = -1.0;
            } else if gc::button_right_held(player) == Some(true) {
                controller.movement.x = 1.0;
            } else {
                controller.movement.x = 0.0;
            }

            //Handle A Button
            if gc::button_a_pressed(player) == Some(true) {
                controller.a = ButtonState::JustPressed
            } else if gc::button_a_released(player) == Some(true) {
                controller.a = ButtonState::JustReleased
            } else if let Some(held) = gc::button_a_held(player) {
                controller.a = match held {
                    true => ButtonState::Pressed,
                    false => ButtonState::Released,
                }
            }

            //Handle B Button
            if gc::button_b_pressed(player) == Some(true) {
                controller.b = ButtonState::JustPressed
            } else if gc::button_b_released(player) == Some(true) {
                controller.b = ButtonState::JustReleased
            } else if let Some(held) = gc::button_b_held(player) {
                controller.b = match held {
                    true => ButtonState::Pressed,
                    false => ButtonState::Released,
                }
            }

            //Handle C Button
            if gc::button_c_pressed(player) == Some(true) {
                controller.c = ButtonState::JustPressed
            } else if gc::button_c_released(player) == Some(true) {
                controller.c = ButtonState::JustReleased
            }
            if let Some(held) = gc::button_c_held(player) {
                controller.c = match held {
                    true => ButtonState::Pressed,
                    false => ButtonState::Released,
                }
            }
        });
}
