use bevy::prelude::{Commands, Deref, DerefMut, Entity, Query, ResMut, Resource, With};
use bevy::utils::HashMap;

use crate::utils::fresh_entity::FreshEntity;
use crate::world::chunk::{ChunkCoord, GameChunk};

#[derive(Resource, Deref, DerefMut, Default)]
/// Entity is meant for GameChunk in this resource
pub struct GameWorld(pub HashMap<ChunkCoord, Entity>);

pub fn bind_fresh_game_chunk_entity_to_game_world(
    mut game_world: ResMut<GameWorld>,
    query: Query<(Entity, &ChunkCoord), (With<GameChunk>, With<FreshEntity>)>,
    mut commands: Commands
) {
    for (entity, coord) in query.iter() {
        game_world.insert(coord.clone(), entity);

        commands.entity(entity).remove::<FreshEntity>();
    }
}