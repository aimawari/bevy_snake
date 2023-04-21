#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::Left => Self::Right,
            Direction::Up => Self::Down,
            Direction::Right => Self::Left,
            Direction::Down => Self::Up,
        }
    }
}
