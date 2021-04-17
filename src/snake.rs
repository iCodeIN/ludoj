use crate::common::{Coordinate, Direction};
use crossterm::{cursor, event, queue, terminal};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::Duration;

const FRAME_RATE: f32 = 10.0;

pub(crate) fn run(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    let frame_delta = Duration::from_secs_f32(1.0 / FRAME_RATE);
    let mut snake = Snake::default();
    let mut food_locations = Vec::new();
    let mut rng = rand::thread_rng();
    let screen_dimensions = terminal::size()?;

    loop {
        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

        if event::poll(frame_delta)? {
            let key_event = if let event::Event::Key(key_event) = event::read()? {
                key_event
            } else {
                continue;
            };

            match key_event {
                event::KeyEvent {
                    code: event::KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => break,

                event::KeyEvent { code, .. } => {
                    snake.direction = match code {
                        event::KeyCode::Up | event::KeyCode::Char('w') => Direction::Up,
                        event::KeyCode::Down | event::KeyCode::Char('s') => Direction::Down,
                        event::KeyCode::Left | event::KeyCode::Char('a') => Direction::Left,
                        event::KeyCode::Right | event::KeyCode::Char('d') => Direction::Right,
                        _ => continue,
                    };
                }
            }
        }

        if rng.gen_bool(0.1) {
            food_locations.push(Coordinate {
                x: rng.gen_range(0..screen_dimensions.0),
                y: rng.gen_range(0..screen_dimensions.1),
            });
        }

        snake.tick(&mut food_locations);

        for segment in &snake.segments {
            queue!(stdout, cursor::MoveTo(segment.x, segment.y))?;
            write!(stdout, "x")?;
        }

        for location in &food_locations {
            queue!(stdout, cursor::MoveTo(location.x, location.y))?;
            write!(stdout, "o")?;
        }

        stdout.flush()?;
    }

    Ok(())
}

struct Snake {
    segments: VecDeque<Coordinate>,
    direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            segments: {
                let mut segments = VecDeque::with_capacity(1);
                segments.push_back(Coordinate { x: 0, y: 0 });
                segments
            },
            direction: Direction::Right,
        }
    }
}

impl Snake {
    fn tick(&mut self, food_locations: &mut Vec<Coordinate>) {
        self.segments
            .push_back(self.segments.back().unwrap().step_in(self.direction));

        let eaten_food_idx = food_locations
            .iter()
            .enumerate()
            .find_map(|(idx, loc)| (self.segments.back().unwrap() == loc).then(|| idx));

        if let Some(eaten_food_idx) = eaten_food_idx {
            food_locations.remove(eaten_food_idx);
        } else {
            // in this case the snake did not eat any food,
            // so we remove the end of the snake.
            self.segments.pop_front();
        }
    }
}
