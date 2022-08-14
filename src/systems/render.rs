use hecs::World;
use rapier2d::{na::Point2, prelude::ColliderHandle};

use crate::{
    components::PlayerId, game::PHYSICS_PIXEL_SCALING, physics_simulation::PhysicsSimulation,
};
use gamercade_rs::{prelude as gc, GraphicsParameters};

fn physics_point_to_screen(point: &Point2<f32>, height: usize) -> (i32, i32) {
    let x = (point.x * PHYSICS_PIXEL_SCALING).round() as i32;
    let y = (height as f32 - (point.y * PHYSICS_PIXEL_SCALING)).round() as i32;

    (x, y)
}

pub fn render_system(
    world: &World,
    physics: &PhysicsSimulation,
    height: usize,
    player_width: u32,
    player_height: u32,
) {
    world
        .query::<(&ColliderHandle, Option<&PlayerId>)>()
        .into_iter()
        .for_each(|(_, (collider, player_id))| {
            if let Some(collider) = physics.collider_set.get(*collider) {
                let aabb = collider.compute_aabb();
                let bottom_left = Point2::new(aabb.mins.x, aabb.maxs.y);
                let top_left = physics_point_to_screen(&bottom_left, height);
                let size = aabb.extents() * PHYSICS_PIXEL_SCALING;

                if let Some(player_id) = player_id {
                    let index = player_id.0 as u8;
                    let gp = GraphicsParameters::default()
                        .palette_index(index)
                        .sprite_sheet_index(index);
                    gc::sprite(gp, 0, top_left.0 - 32, top_left.1 - 10)
                }
                gc::rect(
                    GraphicsParameters::default().color_index(9),
                    top_left.0,
                    top_left.1,
                    size.x as u32,
                    size.y as u32,
                );
            } else {
                gc::console_log("tried to fetch in invalid collider in render_system.")
            }
        });
}
