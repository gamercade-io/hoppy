pub struct Position {
    pub x: PositionValue,
    pub y: PositionValue,
}

pub struct PositionValue {
    pub value: i32,
    pub remainder: f32,
}

impl PositionValue {
    pub const fn new(value: i32) -> Self {
        Self {
            value,
            remainder: 0.0,
        }
    }
}
