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
    prelude::{Color, CrosstermBackend, Frame, Marker, Terminal, Style}, widgets::{canvas::*, *}, text::Line as TextLine
};

use rand::prelude::*;

fn main() -> io::Result<()> {
    App::run()
}

struct Enemy {
    hitbox: Rectangle,
    x: f64,
    y: f64,
    vx: f64,
    // vy: f64
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
    game_over: bool,
    score: usize
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
                        x: 210.0,
                        y: 24.0,
                        width: 2.0,
                        height: 0.0,
                        color: Color::Red,
                    },
                    x: 210.0, 
                    y: 24.0,
                    vx: 1.0,
                    // vy: 0.0
                },
            ],
            game_over: false,
            score: 0
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
                            if !app.game_over {
                                if app.on_ground {
                                    app.pvy = 3.0;
                                    app.on_ground = false
                                }
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

        if self.game_over {
            return
        }

        self.tick_count += 1;

        if self.tick_count % 40 == 0 {

            let mut rng = rand::thread_rng();
            let rand_float: f64 = rng.gen();

            if rand_float > 0.6 {



                self.enemies.push(Enemy { 
                    hitbox: Rectangle {
                        x: 210.0,
                        y: 24.0,
                        width: 2.0,
                        height: 0.0,
                        color: Color::Red,
                    }, 
                    x: 210.0, 
                    y: 24.0, 
                    vx: 1.0 
                })
            }
        }

        if self.py < 24.0 {
            self.py = 24.0;
            self.on_ground = true;
            self.pvy = 0.0;
        }

        if !self.on_ground {
            self.pvy -= 0.15;
        }

        // Handle player collision

        let mut keep: Vec<bool> = vec![];

        for enemy in &mut self.enemies {
            if enemy.collided_with(&self.player) {
                self.game_over = true;
            } else {
                enemy.x -= enemy.vx;
            }

            enemy.hitbox.x = enemy.x;

            if enemy.x < 0.0 {
                keep.push(false);
                self.score += 1;
            } else {
                keep.push(true);
            }
        }

        // below, plus keep line defines a way of choosing which elements in 
        // vector we are going to keep, flagging the others for deletion.
        let mut iter = keep.iter();
        self.enemies.retain(|_| *iter.next().unwrap());

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


                let info_text: TextLine;
                if self.game_over {
                    let contents: &str = "you die";
                    info_text = TextLine::raw(contents);
                } else {
                    let enemies_len: usize = self.enemies.len();
                    let contents_string: String = format!("Existing Enemies: {}", enemies_len).to_string();
                    info_text = TextLine::raw(contents_string);
                }
                ctx.print(40.0,40.0, info_text);
                

                let score_string: String = format!("Score: {}", self.score).to_string();
                let score_text: TextLine = TextLine::raw(score_string);
                ctx.print(180.0,100.0, score_text);

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