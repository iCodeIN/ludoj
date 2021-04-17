#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Coordinate {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Coordinate {
    pub(crate) fn step_in(mut self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }

        self
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}
