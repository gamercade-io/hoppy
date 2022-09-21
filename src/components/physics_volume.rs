pub struct PhysicsVolume {
    pub width: u32,
    pub height: u32,
    pub kind: PhysicsVolumeKind,
}

pub enum PhysicsVolumeKind {
    Actor(ActorState),
    Solid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorState {
    Grounded,
    Airborne,
    Dead,
}

impl Default for ActorState {
    fn default() -> Self {
        Self::Dead
    }
}
