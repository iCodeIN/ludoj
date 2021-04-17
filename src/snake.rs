use crate::common::{Coordinate, Direction};
use crossterm::{cursor, event, queue, terminal};
use std::io::{self, Write};
use std::time::Duration;

const FRAME_RATE: f32 = 60.0;

pub(crate) fn run(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    let frame_delta = Duration::from_secs_f32(1.0 / FRAME_RATE);
    let mut position = Coordinate::default();
    let mut direction = Direction::Right;

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
                    direction = match code {
                        event::KeyCode::Up | event::KeyCode::Char('w') => Direction::Up,
                        event::KeyCode::Down | event::KeyCode::Char('s') => Direction::Down,
                        event::KeyCode::Left | event::KeyCode::Char('a') => Direction::Left,
                        event::KeyCode::Right | event::KeyCode::Char('d') => Direction::Right,
                        _ => continue,
                    };
                }
            }
        }

        position.step_in(direction);

        queue!(stdout, cursor::MoveTo(position.x, position.y))?;
        write!(stdout, "x")?;

        stdout.flush()?;
    }

    Ok(())
}
