use std::io::Stdout;

const SCALE: &[u8] = "..``--//aaww&&WW@@@".as_bytes();
const SCALE_MAX: f64 = (SCALE.len() - 1) as f64;

use crossterm::{
    cursor,
    terminal::{self, disable_raw_mode, enable_raw_mode},
    QueueableCommand,
};

pub fn setup(stdout: &mut Stdout) {
    enable_raw_mode().unwrap();
    stdout.queue(cursor::Hide).unwrap();
    stdout.queue(terminal::EnterAlternateScreen).unwrap();
}

pub fn cleanup(stdout: &mut Stdout) {
    stdout.queue(terminal::LeaveAlternateScreen).unwrap();
    stdout.queue(cursor::Show).unwrap();
    disable_raw_mode().unwrap();
}

pub fn choose_bright(level: f64) -> char {
    SCALE[(SCALE_MAX * level) as usize] as char
}