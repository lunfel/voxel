use bevy::prelude::{Deref, DerefMut, Entity, Resource};
use bevy::utils::HashMap;

use crate::world::chunk::ChunkCoord;

#[derive(Resource, Deref, DerefMut, Default)]
/// Entity is meant for GameChunk in this resource
pub struct GameWorld(pub HashMap<ChunkCoord, Entity>);