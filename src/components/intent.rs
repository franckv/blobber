use crate::movement::Direction;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    None,
    Move(Direction),
    Turn(Direction),
}

#[derive(Clone, Copy, Debug)]
pub struct Intent {
    pub action: Action,
}
