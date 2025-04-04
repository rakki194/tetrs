#![warn(clippy::all, clippy::pedantic)]

pub mod app;
pub mod components;
pub mod config;
pub mod game;
pub mod menu;
pub mod menu_types;
pub mod particles;
pub mod screenshake;
pub mod sound;
pub mod systems;
pub mod ui;

#[cfg(test)]
pub mod tests;

use bevy_ecs::prelude::Resource;
use std::time::{Duration, Instant};

#[derive(Resource, Debug, Clone)]
pub struct Time {
    delta: Duration,
    last_update: Instant,
}

impl Time {
    #[must_use]
    pub fn new() -> Self {
        Self {
            delta: Duration::default(),
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now.duration_since(self.last_update);
        self.last_update = now;
    }

    #[must_use]
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}
