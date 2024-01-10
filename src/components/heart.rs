use std::array;

use crossterm::style::Stylize;

use crate::{screen::Screen, utils::choose_bright};
use std::f64::consts::PI;

        const ANGLE: f64 = 4.0;

fn transform_3d(xy: f64, z: f64) -> f64 {
    let angle_radians = ANGLE / 180.0 * PI;
    return xy / (z * (angle_radians / 2.0).tan());
}

pub struct Heart {
    map: HeartMap,
    cx: usize,
    cy: usize,
    dir: f64,
    size: f64,
    z_offset: f64,
    z_dir: f64,
    rotation: f64,
    rotation_dir: f64,
}

impl Heart {
    pub fn new(cx: usize, cy: usize) -> Self {
        Heart {
            cx,
            cy,
            map: heart_map(),
            dir: 0.01,
            size: 1.0,
            z_dir: 0.1,
            z_offset: 35.0,
            rotation: 0.0,
            rotation_dir: 0.01,
        }
    }

    pub fn update(&mut self) {
        // rotaion
        if self.rotation >= 30000.0 {
            self.rotation = 0.0
        }
        self.rotation += self.rotation_dir;
        // z axis
        // if self.z >= 40.0 {
        //     self.z_dir *= -1.0;
        // }
        // if self.z <= 16.0 {
        //     self.z_dir = self.z_dir.abs()
        // }
        // self.z += self.z_dir;
        // size
        // if self.size >= 1.0 {
        //     self.dir *= -1.0;
        // }
        // if self.size <= 0.7 {
        //     self.dir = self.dir.abs()
        // }
        // self.size += self.dir;
    }

    pub fn draw(&self, screen: &mut Screen) {
        let Self {
            cx,
            cy,
            size,
            z_offset,
            rotation,
            ..
        } = self;

        for scale in (0..(29.0 * size) as usize).map(|i| i as f64 / 29.0) {
            let ch = choose_bright(1.0 - scale / size).red();
            for (x, y) in self.map {
                // scale
                            let x = x * scale;
                let y = y * scale;
                // rotate
                for z in -30..=30 {
                    let (x, y, z) = rotate_3d((x, y, z as f64 / 10.0), (0.0, *rotation, 0.0));
                    // 3d project
                    let z = z + z_offset;
            let ch = choose_bright(1.0 - scale / size).red();
                    let px = transform_3d(x, z);
                    let py = transform_3d(y, z);
                    // center
                    screen.put((*cx as f64 + px) as usize, (*cy as f64 + py) as usize, ch);
                }
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
    array::from_fn(|i| heart_f((i as f64 - 96.0) / 96.0 * PI))
}

fn rotate_3d(coord: (f64, f64, f64), config: (f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = coord;
    let (roll, pitch, yaw) = config;
    let x = yaw.cos() * pitch.cos() * x
        + (yaw.cos() * pitch.sin() * roll.sin() - yaw.sin() * roll.cos()) * y
        + (yaw.cos() * pitch.sin() * roll.cos() + yaw.sin() * roll.sin()) * z;
    let y = yaw.sin() * pitch.cos() * x
        + (yaw.sin() * pitch.sin() * roll.sin() + yaw.cos() * roll.cos()) * y
        + (yaw.sin() * pitch.sin() * roll.cos() - yaw.cos() * roll.sin()) * z;
    let z = -pitch.sin() * x + pitch.cos() * roll.sin() * y + pitch.cos() * roll.cos() * z;
    (x, y, z)
}
