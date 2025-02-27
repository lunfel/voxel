use crate::world::block::CoordSystemIntegerSize;
use bevy::prelude::*;
use serde_derive::Deserialize;
use std::default::Default;
use std::fs;
use std::process::exit;
use toml;

pub const CHUNK_SIZE: CoordSystemIntegerSize = 16;
pub const CHUNK_HEIGHT: CoordSystemIntegerSize = 160;

#[derive(Debug, Deserialize, Resource)]
pub struct Settings {
   pub world: World,
   pub logs: Logs
}

#[derive(Debug, Deserialize, Default)]
pub struct World {
   pub world_dimension: i32,
   pub preload_extra_distance: i32
}

#[derive(Debug, Deserialize, Default)]
pub struct Logs {
   pub triangle_count_enabled: bool,
   pub change_chunk_enabled: bool,
   pub update_as_we_move_enabled: bool
}

impl Default for Settings {
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