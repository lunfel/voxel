use bevy::{prelude::Resource, utils::HashMap};

use crate::utils::point::Point3D;
use crate::world::chunk::ChunkCoord;

use self::chunk::GameChunk;

pub mod block;
pub mod chunk;

pub struct GameCoord(Point3D<i32>);

#[derive(Resource, Default)]
pub struct GameWorld {
    pub chunks: HashMap<ChunkCoord, GameChunk>
}

// impl GameWorld {
//     get_block
// }
