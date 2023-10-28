
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    None,
    Move(Direction),
    Turn(Direction),
}

#[derive(Clone, Copy, Debug)]
pub struct Intent {
    pub action: Action
}
