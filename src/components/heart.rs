use std::array;

use crossterm::style::Stylize;

use crate::{screen::Screen, utils::choose_bright};

pub struct Heart {
    map: HeartMap,
    cx: usize,
    cy: usize,
    dir: f64,
    size: f64,
}

impl Heart {
    pub fn new(cx: usize, cy: usize) -> Self {
        Heart {
            cx,
            cy,
            map: heart_map(),
            dir: 0.01,
            size: 0.7,
        }
    }

    pub fn update(&mut self) {
        if self.size >= 1.0 {
            self.dir *= -1.0;
        }
        if self.size <= 0.7 {
            self.dir = self.dir.abs()
        }
        self.size += self.dir;
    }

    pub fn draw(&self, screen: &mut Screen) {
        let Self { cx, cy, size, .. } = self;

        for scale in (0..(29.0 * size) as usize).map(|i| i as f64 / 29.0) {
            let ch = choose_bright(1.0 - scale / size);
            for (x, y) in self.map {
                let x = (*cx as f64 + x * scale) as usize;
                let y = (*cy as f64 + y * scale) as usize;
                screen.put(x, y, ch.red());
            }
        }
    }
}

type HeartMap = [(f64, f64); 96 * 2];

fn heart_f(t: f64) -> (f64, f64) {
    let x = 16.0 * t.sin().powi(3) * 2.0;
    let y =
        -1.0 * (13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos());
    (x, y)
}

fn heart_map() -> HeartMap {
    array::from_fn(|i| heart_f((i as f64 - 96.0) / 96.0 * 3.0))
}
