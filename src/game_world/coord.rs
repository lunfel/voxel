use bevy::prelude::{Component, Deref, DerefMut};
use bevy_rapier3d::na::Point2;

pub type CoordSystemIntegerSize = i32;

/// ChunkCoord is the coordinate of the chunk in using the
/// value 1 for each chunk. Multiply ChunkCoord by CHUNK_SIZE
/// to get offset in real world
#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct ChunkCoord(Point2<CoordSystemIntegerSize>);