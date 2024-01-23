use crossterm::{
    cursor,
    style::{PrintStyledContent, Stylize},
    terminal, QueueableCommand,
};
use std::io::{Stdout, Write};

pub type Cell = <char as Stylize>::Styled;

pub struct Screen {
    buffer: Vec<Cell>,
    actual: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl Screen {
    pub fn new() -> Self {
        let (width, height) = terminal::size().unwrap();
        let total = (width * height) as usize;
        Self {
            width: width as usize,
            height: height as usize,
            buffer: vec![' '.reset(); total],
            actual: vec!['\0'.reset(); total],
        }
    }

    pub fn put(&mut self, x: usize, y: usize, c: Cell) {
        let index = y * self.width + x;
        if index <= self.buffer.len() - 1 {
            self.buffer[y * self.width + x] = c;
        }
    }

    pub fn clear(&mut self) {
        for c in self.buffer.iter_mut() {
            *c = ' '.reset();
        }
    }

    pub fn flush(&mut self, stdout: &mut Stdout) {
        for i in 0..self.buffer.len() {
            let (bc, ac) = (self.buffer[i], self.actual[i]);
            if bc != ac {
                let (x, y) = (i % self.width, i / self.width);
                stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap();
                stdout.queue(PrintStyledContent(bc)).unwrap();
                self.actual[i] = bc;
            }
        }
        stdout.flush().unwrap();
    }
}
