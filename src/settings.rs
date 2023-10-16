use bevy::prelude::Resource;
pub type CoordSystemIntegerSize = i32;

pub const CHUNK_SIZE: usize = 16;

#[derive(Resource)]
pub struct GameParameters {
    pub chunk_size: CoordSystemIntegerSize
}

impl Default for GameParameters {
    fn default() -> Self {
        Self {
            chunk_size: 16
        }
    }
}