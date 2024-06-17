#![allow(clippy::wildcard_imports)]

use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::Color,
    prelude::Rect,
    prelude::Marker,
    prelude::Frame,
    prelude::Terminal,
    prelude::CrosstermBackend,
    widgets::{canvas::*, *},
};

fn main() -> io::Result<()> {
    App::run()
}

struct App {
    player: Rectangle,
    pvx: f64,
    pvy: f64,
    tick_count: u64,
    marker: Marker,
    ground: Line,
}

impl App {
    fn new() -> Self {
        Self {
            player: Rectangle {
                x: 20.0,
                y: 0.0,
                width: 2.0,
                height: 2.0,
                color: Color::Green,
            },
            ground: Line {
                x1: 0.0,
                y1: 20.0,
                x2: 210.0,
                y2: 20.0,
                color: Color::White,
            },
            pvx: 1.0,
            pvy: 1.0,
            tick_count: 0,
            marker: Marker::Block,
        }
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = Self::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16);
        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        // KeyCode::Down | KeyCode::Char('j') => app.y += 1.0,
                        // KeyCode::Up | KeyCode::Char('k') => app.y -= 1.0,
                        // KeyCode::Right | KeyCode::Char('l') => app.x += 1.0,
                        // KeyCode::Left | KeyCode::Char('h') => app.x -= 1.0,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;

        if self.tick_count % 110 == 0 {
            self.pvx = -self.pvx;
            self.pvy = -self.pvy;
        }

        self.player.x += self.pvx;
        self.player.y += self.pvy;
    }

    fn ui(&self, frame: &mut Frame) {
        frame.render_widget(self.game_canvas(), frame.size());
    }

    fn game_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title(" Jump Boi "))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.player);
                ctx.draw(&self.ground);
            })
            .x_bounds([0.0, 210.0])
            .y_bounds([0.0, 110.0])
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}