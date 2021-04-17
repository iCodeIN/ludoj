#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Coordinate {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Coordinate {
    pub(crate) fn step_in(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}
