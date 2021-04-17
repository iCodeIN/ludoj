use crate::common::{Coordinate, Direction};
use crossterm::{cursor, event, queue, terminal};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::Duration;

const FRAME_RATE: f32 = 60.0;

pub(crate) fn run(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    let frame_delta = Duration::from_secs_f32(1.0 / FRAME_RATE);
    let mut snake = Snake::default();

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

        snake.tick();

        for segment in &snake.segments {
            queue!(stdout, cursor::MoveTo(segment.x, segment.y))?;
            write!(stdout, "x")?;
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
    fn tick(&mut self) {
        self.segments
            .push_back(self.segments.back().unwrap().step_in(self.direction));

        self.segments.pop_front();
    }
}
