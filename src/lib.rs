pub mod macros;
pub mod perceptron;
pub mod types_and_constants;
pub mod components;
pub mod obstacles;
pub mod input;
pub mod assets;
pub mod ecs;
pub mod ui;

pub mod prelude{
    pub use glam::*;

    pub use ggez::{
        conf,
        timer,
        Context,
        event::{self, KeyCode, KeyMods}, GameResult,
        graphics::{self, Color, Image},
        audio::{self, SoundSource},
    };

    pub use rand::Rng;
    pub use rand::rngs::ThreadRng;

    pub use crate::{
        *,
        assets::*,
        obstacles::*,
        components::*,
        ecs::*,
        input::*,
        types_and_constants::*,
        ui::*,
    };
}

use prelude::*;
use rand::rngs::ThreadRng;
use std::io::{Read, Write};
use std::path;
use ggez::filesystem;

pub trait Draw{
    fn draw(&self, ctx: &mut Context, ecs: &ECS, assets: &Assets, entity_id: usize, pos: Vec2, screen_size: Screen2) -> GameResult;
}

pub trait Update{
    fn update(ecs: &mut ECS, assets: &Assets, rng: &mut ThreadRng, entity_id: usize, time: f32, dt: f32);
}

// World and screen positions
pub fn world_to_screen_coords(screen_size: Screen2, point: Vec2) -> Vec2 {
    let x = point.x + screen_size.0 / 2.0;
    let y = screen_size.1 - (point.y + screen_size.1 / 2.0);
    v2!(x, y)
}

pub fn screen_to_world_coords(screen_size: Screen2, point: Vec2) -> Vec2 {
    let x = point.x - screen_size.0 / 2.0;
    let y =  - (point.y - screen_size.1 / 2.0);
    v2!(x, y)
}

pub fn get_time() -> u64{
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

// File stuff
pub fn read_high_score_data(ctx: &mut Context) -> u32 {
    let file_path = path::Path::new("/high_score.txt");
    if !filesystem::is_file(ctx, file_path){
        return 0;
    }
    let mut buffer = Vec::new();
    let mut file = filesystem::open(ctx, file_path).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer).parse().unwrap()
}

pub fn write_high_score_data(ctx: &mut Context, high_score: u32) {
    let file_path = path::Path::new("/high_score.txt");
    let score_str = high_score.to_string();
    let bytes = score_str.as_bytes();
    let mut file = filesystem::create(ctx, file_path).unwrap();
    file.write_all(bytes).unwrap();
}