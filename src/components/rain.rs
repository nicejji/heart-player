use crossterm::style::Stylize;

use crate::{screen::Screen, utils::choose_bright};
use crossterm::style::Color;

use rand::{rngs::ThreadRng, thread_rng, Rng};

struct Drop {
    x: usize,
    size: usize,
    delay: usize,
    cooldown: usize,
    start: usize,
    end: usize,
    max_end: usize,
    max_bright: f64,
}

impl Drop {
    pub fn new(x: usize, max_end: usize, size: usize, delay: usize, max_bright: f64) -> Self {
        Self {
            x,
            max_end,
            size,
            delay,
            cooldown: 0,
            start: 0,
            end: 0,
            max_bright,
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
            let base_bright = (y - self.start) as f64 / (self.end - self.start) as f64;
            let ch = choose_bright(base_bright * self.max_bright);
            let c1 = screen.get(self.x, y).style().foreground_color;
            // foam = 156, 207, 216
            // love = 235, 111, 146
            // base = 25, 23, 36
            let color = match c1 {
                Some(Color::Red) => blend_colors((156, 207, 216), (235, 111, 146), 0.5),
                _ => blend_colors((156, 207, 216), (25, 25, 36), self.max_bright),
            };
            screen.put(self.x, y, ch.with(color));
        }
    }
}

fn blend_colors(c1: (u8, u8, u8), c2: (u8, u8, u8), w: f64) -> Color {
    let (r1, g1, b1) = c1;
    let (r2, g2, b2) = c2;
    Color::Rgb {
        r: (((r1 as f64).powi(2) * w + (r2 as f64).powi(2) * (1.0 - w)).sqrt() as u8),
        g: (((g1 as f64).powi(2) * w + (g2 as f64).powi(2) * (1.0 - w)).sqrt() as u8),
        b: (((b1 as f64).powi(2) * w + (b2 as f64).powi(2) * (1.0 - w)).sqrt() as u8),
    }
}

pub struct Rain {
    drops: Vec<Drop>,
    rng: ThreadRng,
    max_x: usize,
    max_y: usize,
    max_bright: f64,
}

impl Rain {
    pub fn new(max_x: usize, max_y: usize, max_bright: f64) -> Self {
        Self {
            max_x,
            max_y,
            drops: vec![],
            rng: thread_rng(),
            max_bright,
        }
    }

    fn add_drop(&mut self) {
        self.drops.push(Drop::new(
            self.rng.gen_range(0..self.max_x),
            self.max_y,
            self.rng.gen_range(0..9) + 1,
            self.rng.gen_range(0..5),
            self.max_bright,
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
