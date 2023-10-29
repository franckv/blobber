use crate::movement::Direction;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    None,
    Move(Direction),
    Turn(Direction),
    Look((f32, f32)),
    ControlCamera(bool),
}

#[derive(Clone, Copy, Debug)]
pub struct Intent {
    pub action: Action,
}
