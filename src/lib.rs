#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use anyhow::Error;
use crankstart::display::Display;
use crankstart::{crankstart_game, Game, Playdate};
use crankstart::{graphics::*, log_to_console};
use euclid::Point2D;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

const STARS: usize = 1000;

#[inline(always)]
fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    start2 + (stop2 - start2) * ((value - start1) / (stop1 - start1))
}

#[inline(always)]
fn random(value: u32, min: f32, max: f32) -> f32 {
    (value as f32 % (max - min)) + min
}

#[derive(Copy, Clone)]
struct Star {
    x: f32,
    y: f32,
    z: f32,
    pz: f32,
}

impl Star {
    #[inline(always)]
    fn new(rng: &mut SmallRng) -> Self {
        let z = random(rng.next_u32(), 0.0, LCD_COLUMNS as f32);
        Self {
            x: random(rng.next_u32(), -(LCD_COLUMNS as f32), LCD_COLUMNS as f32),
            y: random(rng.next_u32(), -(LCD_ROWS as f32), LCD_ROWS as f32),
            z,
            pz: z,
        }
    }

    #[inline(always)]
    fn update(&mut self, rng: &mut SmallRng, speed: f32) {
        self.z -= speed;
        if self.z < 1.0 {
            self.z = LCD_COLUMNS as f32;
            self.x = (rng.next_u32() % LCD_COLUMNS) as f32;
            self.y = (rng.next_u32() % LCD_ROWS) as f32;
            self.pz = self.z;
        }
    }

    #[inline(always)]
    fn show(&mut self) {
        let sx = map(self.x / self.z, 0.0, 1.0, 0.0, LCD_COLUMNS as f32);
        let sy = map(self.y / self.z, 0.0, 1.0, 0.0, LCD_ROWS as f32);

        let r = map(self.z, 0.0, LCD_COLUMNS as f32, 4.0, 0.0);

        let px = map(self.x / self.pz, 0.0, 1.0, 0.0, LCD_COLUMNS as f32);
        let py = map(self.y / self.pz, 0.0, 1.0, 0.0, LCD_ROWS as f32);

        self.pz = self.z;

        log_to_console!("{} {} {} {} {}", px, py, sx, sy, r);

        Graphics::get()
            .draw_line(
                Point2D::new(px as i32, py as i32),
                Point2D::new(sx as i32, sy as i32),
                r as i32,
                LCDColor::Solid(LCDSolidColor::kColorWhite),
            )
            .unwrap();
    }
}

struct Starfield {
    rng: SmallRng,
    stars: [Star; STARS],
}

impl Starfield {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        Display::get().set_refresh_rate(50.0)?;
        let mut rng = SmallRng::seed_from_u64(0);
        let stars: [Star; STARS] = [Star::new(&mut rng); STARS];
        Ok(Box::new(Self { rng, stars }))
    }
}

impl Game for Starfield {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        Graphics::get().clear(LCDColor::Solid(LCDSolidColor::kColorBlack))?;
        for star in &mut self.stars {
            star.update(&mut self.rng, 5.0);
            star.show();
        }
        Ok(())
    }
}

crankstart_game!(Starfield);
