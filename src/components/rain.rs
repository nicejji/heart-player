use crossterm::style::Stylize;

use crate::{screen::Screen, utils::choose_bright};

use rand::{rngs::ThreadRng, thread_rng, Rng};

struct Drop {
    x: usize,
    size: usize,
    delay: usize,
    cooldown: usize,
    start: usize,
    end: usize,
    max_end: usize,
}

impl Drop {
    pub fn new(x: usize, max_end: usize, size: usize, delay: usize) -> Self {
        Self {
            x,
            max_end,
            size,
            delay,
            cooldown: 0,
            start: 0,
            end: 0,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.start < self.max_end - 1
    }

    pub fn update(&mut self) {
        if self.cooldown >= self.delay {
            self.cooldown = 0;
            if self.end >= self.size && self.start < self.max_end {
                self.start += 1;
            }
            if self.end < self.max_end {
                self.end += 1;
            }
        } else {
            self.cooldown += 1;
        }
    }

    pub fn draw(&self, screen: &mut Screen) {
        for y in self.start..self.end {
            let ch = choose_bright((y - self.start) as f64 / (self.end - self.start) as f64);
            screen.put(self.x, y, ch.blue());
        }
    }
}

pub struct Rain {
    drops: Vec<Drop>,
    rng: ThreadRng,
    max_x: usize,
    max_y: usize,
}

impl Rain {
    pub fn new(max_x: usize, max_y: usize) -> Self {
        Self {
            max_x,
            max_y,
            drops: vec![],
            rng: thread_rng(),
        }
    }

    fn add_drop(&mut self) {
        self.drops.push(Drop::new(
            self.rng.gen_range(0..self.max_x),
            self.max_y,
            self.rng.gen_range(0..9) + 1,
            self.rng.gen_range(0..5),
        ))
    }

    pub fn update(&mut self) {
        let roll: f64 = self.rng.gen();
        if roll < 0.3 {
            self.add_drop()
        }
        for drop in self.drops.iter_mut() {
            drop.update();
        }
        self.drops.retain(|d| d.is_valid());
    }

    pub fn draw(&self, screen: &mut Screen) {
        for drop in self.drops.iter() {
            drop.draw(screen);
        }
    }
}
