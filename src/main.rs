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

struct Enemy {
    hitbox: Rectangle,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64
}

impl Enemy {
    fn collided_with(&self, player: &Rectangle) -> bool {

        /*

            Player: (y: 20, x: 10)
            Enemy (self): (y: 19, x: 10)

            -- If y is within 1 and x is within 1

            -- Subtract enemy x from player x
            -- 10 - 10 = 0
            -- Subtract enemy y from player y
            -- 20 - 19 = 1
            
            if difference is between -2 and +2 for both, collision
        */

        let xdiff = player.x - self.x;
        let ydiff = player.y - self.y;

        if -2.0 < xdiff && xdiff < 2.0 {
            if -2.0 < ydiff && ydiff < 2.0 {
                return true
            }
        }

        false

    }
}

struct App {
    player: Rectangle,
    on_ground: bool,
    py: f64,
    pvy: f64,
    tick_count: u64,
    marker: Marker,
    ground: Line,
    enemies: Vec<Enemy>,
    collision_exists: bool
}

impl App {
    fn new() -> Self {
        Self {
            player: Rectangle {
                x: 20.0,
                y: 24.0,
                width: 2.0,
                height: 0.0,
                color: Color::Green,
            },
            ground: Line {
                x1: 0.0,
                y1: 20.0,
                x2: 210.0,
                y2: 20.0,
                color: Color::White,
            },
            on_ground: true,
            pvy: 0.0,
            py: 24.0,
            tick_count: 0,
            marker: Marker::Block,
            enemies: vec![
                Enemy {
                    hitbox: Rectangle {
                        x: 80.0,
                        y: 24.0,
                        width: 2.0,
                        height: 0.0,
                        color: Color::Red,
                    },
                    x: 80.0, 
                    y: 24.0,
                    vx: 0.0,
                    vy: 0.0
                },
                Enemy {
                    hitbox: Rectangle {
                        x: 120.0,
                        y: 24.0,
                        width: 2.0,
                        height: 0.0,
                        color: Color::Red,
                    },
                    x: 80.0, 
                    y: 24.0,
                    vx: 0.0,
                    vy: 0.0
                },
            ],
            collision_exists: false
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
                        KeyCode::Up | KeyCode::Char('w') => {
                            if app.on_ground {
                                app.pvy = 3.0;
                                app.on_ground = false
                            }
                        },
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

        if self.py < 24.0 {
            self.py = 24.0;
            self.on_ground = true;
            self.pvy = 0.0;
        }

        if !self.on_ground {
            self.pvy -= 0.1;
        }

        // Handle player collision

        for enemy in &self.enemies {
            if enemy.collided_with(&self.player) {
                println!("COLLIDED")
                self.collision_exists = true;

                // render a label with 'ded' 
            }
        }

        self.py = self.py + self.pvy;
        self.player.y = self.py;

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

                for enemy in &self.enemies {
                    ctx.draw(&enemy.hitbox)
                }
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