use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::world::chunk::ChunkCoord;

#[derive(Resource, Deref, DerefMut, Default)]
/// Entity is meant for GameChunk in this resource
pub struct GameWorld(pub HashMap<ChunkCoord, Entity>);

#[derive(Component, Default, Debug)]
pub struct PendingAdditionToGameWorld;

#[derive(Component, Default, Debug)]
pub struct ChunkKeepAlive {
    // Stores the game Time.elapsed to compare staleness
    pub last_touch: f32
}

pub fn add_pending_chunks_to_game_world(
    query: Query<(Entity, &ChunkCoord), With<PendingAdditionToGameWorld>>,
    mut game_world: ResMut<GameWorld>,
    mut commands: Commands
) {
    for (entity, chunk_coord) in query.iter() {
        game_world.insert(*chunk_coord, entity);

        commands.entity(entity).remove::<PendingAdditionToGameWorld>();
    }
}