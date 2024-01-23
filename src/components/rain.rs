use crossterm::style::Stylize;

use crate::{
    screen::Screen,
    utils::{choose_bright, UpdateDebouncer},
};
use crossterm::style::Color;

use rand::{rngs::ThreadRng, thread_rng, Rng};

struct Drop {
    start: usize,
    end: usize,
    x: usize,
    max_y: usize,
    max_x: usize,
    size: usize,
    brightness: f64,
    debouncer: UpdateDebouncer,
}

impl Drop {
    pub fn new(
        x: usize,
        max_x: usize,
        max_y: usize,
        size: usize,
        brightness: f64,
        speed: f64,
    ) -> Self {
        Self {
            start: 0,
            end: 0,
            x,
            max_y,
            max_x,
            size,
            debouncer: UpdateDebouncer::new(speed),
            brightness,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.start < self.end
    }

    pub fn update(&mut self) {
        if !self.debouncer.next().unwrap() {
            return;
        }
        if self.start < self.max_y && self.end >= self.size {
            self.start += 1;
        }
        if self.end < self.max_y {
            self.end += 1;
        }
    }

    pub fn draw(&self, screen: &mut Screen) {
        for y in self.start..=self.end {
            let base_bright = (y - self.start) as f64 / (self.end - self.start) as f64;
            let ch = choose_bright(base_bright * self.brightness).with(Color::Blue);
            screen.put(self.x, y, ch);
        }
        if self.end == self.max_y {
            let w = self.size.saturating_sub(self.end - self.start) / 2;
            for x in 1..=w {
                let ch = choose_bright((1.0 - x as f64 / w as f64) / 2.0).with(Color::Blue);
                screen.put((self.x + x).min(self.max_x), self.end, ch);
                screen.put(self.x.saturating_sub(x), self.end, ch);
            }
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
    pub fn new(screen: &Screen) -> Self {
        Self {
            max_x: screen.width - 1,
            max_y: screen.height - 1,
            drops: vec![],
            rng: thread_rng(),
        }
    }

    fn add_drop(&mut self) {
        self.drops.push(Drop::new(
            self.rng.gen_range(0..=self.max_x),
            self.max_x,
            self.max_y,
            self.rng.gen_range(3..=10),
            self.rng.gen::<f64>(),
            self.rng.gen_range(3..=10) as f64 / 10.0,
        ))
    }

    pub fn update(&mut self) {
        if self.rng.gen_bool(0.7) {
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
