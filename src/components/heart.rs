use crossterm::style::Stylize;

use crate::screen::Screen;
use std::f64::consts::PI;

const HEART_X_MAX: f64 = 2.0;

pub struct Heart {
    cx: usize,
    cy: usize,
}

impl Heart {
    pub fn new(screen: &Screen) -> Self {
        Heart {
            cx: screen.width / 2,
            cy: screen.height / 2,
        }
    }

    pub fn update(&mut self) {
        // :TODO: Heart animation
    }

    pub fn draw(&self, screen: &mut Screen) {
        let Self { cx, cy, .. } = self;

        let points = (*cx as f64 / 2.0) as i32;
        let scale = points as f64 / HEART_X_MAX;
        let actual_y = |y: f64| (-y * scale / 2.0 + *cy as f64) as usize;
        for x in -points..=points {
            let (y_top, y_bottom) = heart_f(HEART_X_MAX * x as f64 / points as f64);
            let (mut y_top, mut y_bottom) = (actual_y(y_top), actual_y(y_bottom));
            if x == 0 {
                y_top -= 2;
                y_bottom -= 1;
            }
            let x = (*cx as i32 + x) as usize;
            let ch = '.'.blue();
            for y in y_top..=y_bottom {
                screen.put(x, y, ch);
            }
        }
    }
}

fn heart_f(x: f64) -> (f64, f64) {
    let y_top = (1.0 - (x.abs() - 1.0).powi(2)).sqrt();
    let y_bottom = (1.0 - x.abs()).acos() - PI;
    return (y_top + 0.8, y_bottom + 0.8);
}
