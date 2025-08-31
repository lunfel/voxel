pub type CoordSystemIntegerSize = i32;

pub const CHUNK_SIZE: CoordSystemIntegerSize = 16;
pub const CHUNK_HEIGHT: CoordSystemIntegerSize = 80;

pub const MAX_OFFSET: CoordSystemIntegerSize = CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE;

use crate::chunk::noise::Noise;
use bevy::prelude::*;
use serde_derive::Deserialize;
use std::default::Default;
use std::fs;
use std::process::exit;
use toml;

#[derive(Debug, Deserialize, Asset, TypePath, Clone)]
pub struct GameSettings {
    pub world: World,
    pub logs: Logs,
    pub procedural: Procedural,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameSettingsHandle {
    pub handle: Handle<GameSettings>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct World {
    pub world_dimension: i32,
    pub preload_extra_distance: i32,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Logs {
    pub triangle_count_enabled: bool,
    pub change_chunk_enabled: bool,
    pub update_as_we_move_enabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Procedural {
    pub base_noise: Noise,
    pub block_noise: Noise,
}

impl Default for GameSettings {
    fn default() -> Self {
        // Variable that holds the filename as a `&str`.
        let filename = "game.toml";

        // Read the contents of the file using a `match` block
        // to return the `data: Ok(c)` as a `String`
        // or handle any `errors: Err(_)`.
        let contents = match fs::read_to_string(filename) {
            // If successful return the files text as `contents`.
            // `c` is a local variable.
            Ok(c) => c,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                eprintln!("Could not read file `{}`", filename);
                // Exit the program with exit code `1`.
                exit(1);
            }
        };

        // Use a `match` block to return the
        // file `contents` as a `Data struct: Ok(d)`
        // or handle any `errors: Err(_)`.
        match toml::from_str(&contents) {
            // If successful, return data as `Data` struct.
            // `d` is a local variable.
            Ok(d) => d,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                panic!("Unable to load data from `{}`", filename);
            }
        }
    }
}
