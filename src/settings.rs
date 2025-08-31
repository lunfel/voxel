pub type CoordSystemIntegerSize = i32;

pub const CHUNK_SIZE: CoordSystemIntegerSize = 16;
pub const CHUNK_HEIGHT: CoordSystemIntegerSize = 80;

pub const MAX_OFFSET: CoordSystemIntegerSize = CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE;

use crate::chunk::noise::Noise;
use bevy::prelude::*;
use serde_derive::Deserialize;
use std::default::Default;

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
    pub change_chunk_enabled: bool,
    pub update_as_we_move_enabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Procedural {
    pub base_noise: Noise,
    pub block_noise: Noise,
}
