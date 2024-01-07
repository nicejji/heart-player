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

use components::heart::Heart;
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
}
impl State {
    fn new(heart: Heart) -> Self {
        Self { heart }
    }
    fn update(&mut self) {
        self.heart.update();
    }
}

fn render(screen: &mut Screen, stdout: &mut Stdout, state: &State) {
    screen.clear();
    state.heart.draw(screen);
    screen.flush(stdout);
}

fn main() {
    let mut stdout = stdout();
    setup(&mut stdout);

    let (width, height) = terminal::size().unwrap();
    let mut screen = Screen::new(width as usize, height as usize);

    let mut state = State::new(Heart::new(width as usize / 2, height as usize / 2));

    let fps = 60;

    loop {
        render(&mut screen, &mut stdout, &state);
        state.update();
        handle_keys(&mut stdout, 1000 / fps);
    }
}
