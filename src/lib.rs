#![no_std]
#![feature(const_fn_floating_point_arithmetic)]

extern crate alloc;

use alloc::boxed::Box;
use anyhow::Error;
use crankstart::display::Display;
use crankstart::graphics::*;
use crankstart::system::System;
use crankstart::{crankstart_game, Game, Playdate};
use euclid::Point2D;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

// Any value above ~600 will cause stack overflow
// Starfield struct is 9632 bytes, Playdate's stack size is 61800 bytes
const STARS: usize = 600;

const WIDTH: f32 = LCD_COLUMNS as f32;
const HEIGHT: f32 = LCD_ROWS as f32;

#[inline(always)]
const fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    start2 + (stop2 - start2) * ((value - start1) / (stop1 - start1))
}

#[inline(always)]
const fn random(value: u32, min: f32, max: f32) -> f32 {
    let fvalue = value as f32 / 100000.0;
    fvalue % (max - min) + min
}

#[inline(always)]
const fn black() -> LCDColor {
    LCDColor::Solid(LCDSolidColor::kColorBlack)
}

#[inline(always)]
const fn white() -> LCDColor {
    LCDColor::Solid(LCDSolidColor::kColorWhite)
}

#[derive(Default, Copy, Clone)]
struct Star {
    x: f32,
    y: f32,
    z: f32,
    pz: f32,
}

impl Star {
    #[inline(always)]
    fn new(rng: &mut SmallRng) -> Self {
        let z = random(rng.next_u32(), 0.0, WIDTH);
        Self {
            x: random(rng.next_u32(), -WIDTH, WIDTH),
            y: random(rng.next_u32(), -HEIGHT, HEIGHT),
            z,
            pz: z,
        }
    }

    #[inline(always)]
    fn update(&mut self, rng: &mut SmallRng, speed: f32) -> Result<(), Error> {
        self.z -= speed;
        if self.z < 1.0 {
            self.z = LCD_COLUMNS as f32;
            self.x = random(rng.next_u32(), -WIDTH, WIDTH);
            self.y = random(rng.next_u32(), -HEIGHT, HEIGHT);
            self.pz = self.z;
        }
        Ok(())
    }

    #[inline(always)]
    fn show(&mut self) -> Result<(), Error> {
        let sx = map((self.x / self.z) + 0.5, 0.0, 1.0, 0.0, WIDTH);
        let sy = map((self.y / self.z) + 0.5, 0.0, 1.0, 0.0, HEIGHT);

        let r = map(self.z, 0.0, WIDTH, 4.0, 0.0);

        let px = map((self.x / self.pz) + 0.5, 0.0, 1.0, 0.0, WIDTH);
        let py = map((self.y / self.pz) + 0.5, 0.0, 1.0, 0.0, HEIGHT);

        self.pz = self.z;

        Graphics::get().draw_line(
            Point2D::new(px as i32, py as i32),
            Point2D::new(sx as i32, sy as i32),
            r as i32,
            white(),
        )?;
        Ok(())
    }
}

struct Starfield {
    rng: SmallRng,
    stars: [Star; STARS],
}

impl Starfield {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        Display::get().set_refresh_rate(50.0)?;
        let (_, time) = System::get().get_seconds_since_epoch()?;
        let mut rng = SmallRng::seed_from_u64(time as u64);
        let mut stars: [Star; STARS] = [Star::default(); STARS];
        for star in stars.iter_mut().take(STARS) {
            *star = Star::new(&mut rng);
        }
        Ok(Box::new(Self { rng, stars }))
    }
}

impl Game for Starfield {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        Graphics::get().clear(black())?;
        let crank_change = System::get().get_crank_change()?;
        for star in &mut self.stars {
            star.update(&mut self.rng, crank_change.max(0.0))?;
            star.show()?;
        }
        // System::get().draw_fps(0, 0)?;
        Ok(())
    }
}

crankstart_game!(Starfield);
