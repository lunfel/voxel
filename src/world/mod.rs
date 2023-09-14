use bevy::{prelude::Resource, utils::HashMap};

use crate::utils::point::Point3D;

use self::chunk::GameChunk;

pub mod block;
pub mod chunk;

pub struct GameCoord(Point3D<i32>);

#[derive(Resource, Default)]
pub struct GameWorld {
    pub chunks: HashMap<Point3D<usize>, GameChunk>
}

// impl GameWorld {
//     get_block
// }
