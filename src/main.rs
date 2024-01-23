mod components;
mod screen;
mod utils;

use components::{heart::Heart, rain::Rain};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use screen::Screen;
use std::{io::stdout, process::exit, time::Duration};
use utils::{cleanup, setup};

const FPS: u64 = 60;

fn main() {
    let mut stdout = stdout();
    setup(&mut stdout);

    let mut screen = Screen::new();

    let mut rain = Rain::new(&screen);
    let mut heart = Heart::new(&screen);

    let mut paused = false;

    loop {
        if !paused {
            // render
            screen.clear();
            rain.draw(&mut screen);
            heart.draw(&mut screen);
            screen.flush(&mut stdout);

            // update state
            rain.update();
            heart.update();
        }

        // handle key events
        if poll(Duration::from_millis(1000 / FPS)).unwrap() {
            match read().unwrap() {
                Event::Key(k) => match k {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        cleanup(&mut stdout);
                        exit(0);
                    }
                    KeyEvent {
                        code: KeyCode::Char(' '),
                        ..
                    } => {
                        paused = !paused;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
