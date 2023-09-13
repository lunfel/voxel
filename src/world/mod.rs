use bevy::{prelude::Resource, utils::HashMap};

use crate::utils::point::Point3D;

use self::chunk::GameChunk;

pub mod block;
pub mod chunk;

#[derive(Resource, Default)]
pub struct GameWorld {
    pub chunks: HashMap<Point3D<usize>, GameChunk>
}
