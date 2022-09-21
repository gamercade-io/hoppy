use std::ops::Range;

use gamercade_rs::prelude::GraphicsParameters;

pub struct Sprite {
    pub x_offset: u32,
    pub y_offset: u32,
    pub kind: SpriteKind,
}

pub enum SpriteKind {
    Static(GraphicsParameters),
    Animated(AnimatedSprite),
}

pub struct AnimatedSprite {
    pub palette: u8,
    pub sprite_sheet: u8,
    pub sprite: u8,
    pub flip_x: bool,
    pub flip_y: bool,

    pub current_anim: AnimState,
    pub frame_count: u8,
}

impl SpriteKind {
    pub fn as_parameters(&self) -> GraphicsParameters {
        match self {
            SpriteKind::Static(gp) => *gp,
            SpriteKind::Animated(sprite) => GraphicsParameters::new()
                .palette_index(sprite.palette)
                .sprite_sheet_index(sprite.sprite_sheet)
                .sprite_index(sprite.sprite)
                .flip_x(sprite.flip_x)
                .flip_y(sprite.flip_y),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum AnimState {
    Idle,
    Running,
}

impl AnimState {
    pub const IDLE_RANGE: Range<u8> = 0..14;
    pub const RUN_RANGE: Range<u8> = 15..19;

    pub const fn get_range(&self) -> Range<u8> {
        match self {
            AnimState::Idle => Self::IDLE_RANGE,
            AnimState::Running => Self::RUN_RANGE,
        }
    }
}

pub const TICKS_PER_FRAME: u8 = 6;
