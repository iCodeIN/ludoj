use crate::common::{Coordinate, Direction};
use crossterm::{cursor, event, queue, terminal};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::Duration;

const FRAME_RATE: f32 = 10.0;

pub(crate) fn run(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    let frame_delta = Duration::from_secs_f32(1.0 / FRAME_RATE);

    let screen_dimensions = terminal::size()?;
    let screen_dimensions = Coordinate {
        x: screen_dimensions.0,
        y: screen_dimensions.1,
    };

    let mut game = Game::new(screen_dimensions);

    loop {
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
                    game.snake.direction = match code {
                        event::KeyCode::Up | event::KeyCode::Char('w') => Direction::Up,
                        event::KeyCode::Down | event::KeyCode::Char('s') => Direction::Down,
                        event::KeyCode::Left | event::KeyCode::Char('a') => Direction::Left,
                        event::KeyCode::Right | event::KeyCode::Char('d') => Direction::Right,
                        _ => continue,
                    };
                }
            }
        };

        game.tick();
        game.draw(stdout)?;
    }

    Ok(())
}

struct Game {
    snake: Snake,
    food_location: Coordinate,
    rng: ThreadRng,
    screen_dimensions: Coordinate,
}

impl Game {
    fn new(screen_dimensions: Coordinate) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            snake: Snake::default(),
            food_location: generate_food_location(&mut rng, screen_dimensions),
            rng,
            screen_dimensions,
        }
    }

    fn tick(&mut self) {
        let tick_record = self.snake.tick(self.food_location);

        if tick_record.ate_food {
            self.regenerate_food_location();
        }
    }

    fn draw(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

        for segment in &self.snake.segments {
            queue!(stdout, cursor::MoveTo(segment.x, segment.y))?;
            write!(stdout, "x")?;
        }

        queue!(
            stdout,
            cursor::MoveTo(self.food_location.x, self.food_location.y),
        )?;
        write!(stdout, "o")?;

        stdout.flush()?;

        Ok(())
    }

    fn regenerate_food_location(&mut self) {
        self.food_location = generate_food_location(&mut self.rng, self.screen_dimensions);
    }
}

fn generate_food_location(rng: &mut ThreadRng, screen_dimensions: Coordinate) -> Coordinate {
    Coordinate {
        x: rng.gen_range(0..screen_dimensions.x),
        y: rng.gen_range(0..screen_dimensions.y),
    }
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
    fn tick(&mut self, food_location: Coordinate) -> TickRecord {
        self.segments
            .push_back(self.segments.back().unwrap().step_in(self.direction));

        let ate_food = *self.segments.back().unwrap() == food_location;

        if !ate_food {
            self.segments.pop_front();
        }

        TickRecord { ate_food }
    }
}

struct TickRecord {
    ate_food: bool,
}
