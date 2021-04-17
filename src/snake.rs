use crossterm::{cursor, event, queue, terminal};
use std::io::{self, Write};
use std::time::Duration;

const FRAME_RATE: f32 = 60.0;

pub(crate) fn run(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    let frame_delta = Duration::from_secs_f32(1.0 / FRAME_RATE);
    let mut snake_position = (0, 0);

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

                event::KeyEvent { code, .. } => match code {
                    event::KeyCode::Up | event::KeyCode::Char('w') => snake_position.1 -= 1,
                    event::KeyCode::Down | event::KeyCode::Char('s') => snake_position.1 += 1,
                    event::KeyCode::Left | event::KeyCode::Char('a') => snake_position.0 -= 1,
                    event::KeyCode::Right | event::KeyCode::Char('d') => snake_position.0 += 1,
                    _ => {}
                },
            }
        }

        queue!(stdout, cursor::MoveTo(snake_position.0, snake_position.1))?;
        write!(stdout, "x")?;

        stdout.flush()?;
    }

    Ok(())
}
