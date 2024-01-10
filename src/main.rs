mod components;
mod screen;
mod utils;

use std::{
    io::{stdout, Stdout},
    process::exit,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self},
};

use components::{heart::Heart, rain::Rain};
use screen::Screen;
use utils::{cleanup, setup};

fn handle_keys(stdout: &mut Stdout, timeout_ms: u64) {
    if poll(Duration::from_millis(timeout_ms)).unwrap() {
        match read().unwrap() {
            Event::Key(k) => match k {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    cleanup(stdout);
                    exit(0);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

struct State {
    heart: Heart,
    rain_bg: Rain,
    rain_fg: Rain,
}
impl State {
    fn new(heart: Heart, rain_bg: Rain, rain_fg: Rain) -> Self {
        Self {
            heart,
            rain_bg,
            rain_fg,
        }
    }
    fn update(&mut self) {
        self.rain_bg.update();
        self.heart.update();
        self.rain_fg.update();
    }
}

fn render(screen: &mut Screen, stdout: &mut Stdout, state: &State) {
    screen.clear();
    state.rain_bg.draw(screen);
    state.heart.draw(screen);
    state.rain_fg.draw(screen);
    screen.flush(stdout);
}

fn main() {
    let mut stdout = stdout();
    setup(&mut stdout);

    let (width, height) = terminal::size().unwrap();
    let mut screen = Screen::new(width as usize, height as usize);

    let mut state = State::new(
        Heart::new(width as usize / 2, height as usize / 2),
        Rain::new(width as usize, height as usize, 0.3),
        Rain::new(width as usize, height as usize, 1.0),
    );

    let fps = 60;

    loop {
        render(&mut screen, &mut stdout, &state);
        state.update();
        handle_keys(&mut stdout, 1000 / fps);
    }
}
