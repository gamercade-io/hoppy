#[derive(Default)]
pub struct Controller {
    pub movement_x: f32,
    pub movement_y: f32,
    pub a: ButtonState,
    pub b: ButtonState,
    pub c: ButtonState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
    JustReleased,
    JustPressed,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Released
    }
}
