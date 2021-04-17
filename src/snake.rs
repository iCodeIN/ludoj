use crossterm::{event, queue, terminal};
use std::io;
use std::time::Duration;

const FRAME_RATE: f32 = 60.0;

pub(crate) fn run(mut stdout: io::Stdout) -> anyhow::Result<()> {
    let frame_delta = Duration::from_secs_f32(1.0 / FRAME_RATE);

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
                    event::KeyCode::Up | event::KeyCode::Char('w') => {}
                    event::KeyCode::Down | event::KeyCode::Char('s') => {}
                    event::KeyCode::Left | event::KeyCode::Char('a') => {}
                    event::KeyCode::Right | event::KeyCode::Char('d') => {}
                    _ => {}
                },
            }
        };
    }

    Ok(())
}
