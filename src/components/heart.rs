use crossterm::style::Stylize;

use crate::{screen::Screen, utils::choose_bright};
use std::f64::consts::PI;

const HEART_X_MAX: f64 = 2.0;
const GRAD_PRECISION: usize = 10;

pub struct Heart {
    cx: usize,
    cy: usize,
    size: f64,
    size_mod: f64,
}

impl Heart {
    pub fn new(screen: &Screen) -> Self {
        Heart {
            cx: screen.width / 2,
            cy: screen.height / 2,
            size: 1.0,
            size_mod: 0.01,
        }
    }

    pub fn update(&mut self) {
        if self.size >= 1.0 {
            self.size_mod = -self.size_mod.abs()
        }
        if self.size <= 0.7 {
            self.size_mod = self.size_mod.abs()
        }
        self.size += self.size_mod;
        // :TODO: Animate more (probably 3D ...)
    }

    pub fn draw(&self, screen: &mut Screen) {
        let width = self.cx as f64 / 2.5 * self.size;
        for g_stop in (1..=GRAD_PRECISION).rev() {
            let g_stop = g_stop as f64 / GRAD_PRECISION as f64;
            let ch = choose_bright(1.0 - g_stop).red();
            let points = width * g_stop;
            let scale = points / HEART_X_MAX;
            let actual_y = |y: f64| (-y * scale / 2.0 + self.cy as f64) as usize;
            for x in 0..=points as usize {
                let (y_top, y_bottom) = heart_f(HEART_X_MAX * x as f64 / points as f64);
                let (mut y_top, mut y_bottom) = (actual_y(y_top), actual_y(y_bottom));
                if x == 0 {
                    y_top -= 2;
                    y_bottom -= 1;
                }
                for y in y_top..=y_bottom {
                    screen.put(self.cx - x, y, ch);
                    screen.put(self.cx + x, y, ch);
                }
            }
        }
    }
}

fn heart_f(x: f64) -> (f64, f64) {
    let y_top = (1.0 - (x.abs() - 1.0).powi(2)).sqrt();
    let y_bottom = (1.0 - x.abs()).acos() - PI;
    return (y_top + 0.8, y_bottom + 0.8);
}
