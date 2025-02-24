use std::collections::HashMap;
use crate::logging::LogIntervalTimer;
use crate::player::control::ThePlayer;
use crate::settings::Settings;
use crate::world::world_generation::{generate_chunk, spawn_chunk_from_data, BlockMaterial, ChunkData, WorldGenerationState};
use bevy::prelude::*;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use crate::world::chunk::{world_transform_to_chunk_coordinates, ChunkCoord};
use crate::world::game_world::{ChunkKeepAlive, GameWorld};

#[derive(Resource, Debug, Default)]
pub struct ChunkGenerationTaskMap {
    chunks: HashMap<ChunkCoord, Task<ChunkData>>
}

#[derive(Resource, Deref, DerefMut, Debug, Clone, Default)]
pub struct PlayerLastChunkCoord(ChunkCoord);

#[derive(Event)]
pub struct PlayerChangedChunkCoordEvent {
    pub new_position: ChunkCoord,
    pub previous_position: ChunkCoord,
}

pub fn check_for_player_chunk_position_update(
    world_generation_state: ResMut<WorldGenerationState>,
    player_last_chunk_coord: Res<PlayerLastChunkCoord>,
    log_interval: Res<LogIntervalTimer>,
    player: Query<&Transform, With<ThePlayer>>,
    mut ev_changed_coord: EventWriter<PlayerChangedChunkCoordEvent>,
) {
    if let Ok(transform) = player.get_single() {
        let player_chunk_coord: ChunkCoord = world_transform_to_chunk_coordinates(transform);

        if player_chunk_coord != player_last_chunk_coord.0 {
            ev_changed_coord.send(PlayerChangedChunkCoordEvent {
                new_position: player_chunk_coord,
                previous_position: player_last_chunk_coord.0
            });
        }
    }
}

pub fn update_player_last_chunk_coord(
    mut player_last_chunk_coord: ResMut<PlayerLastChunkCoord>,
    mut ev_changed_coord: EventReader<PlayerChangedChunkCoordEvent>,
    settings: Res<Settings>,
) {
    if !settings.logs.change_chunk_enabled {
        return;
    }

    for ev in ev_changed_coord.read() {
        info!("Player is now in chunk {}", *ev.new_position);

        player_last_chunk_coord.0 = ev.new_position;
    }
}

pub fn begin_generating_map_chunks(
    mut ev_changed_coord: EventReader<PlayerChangedChunkCoordEvent>,
    mut generation_tasks: ResMut<ChunkGenerationTaskMap>,
    game_world: Res<GameWorld>,
    settings: Res<Settings>
) {
    if ev_changed_coord.is_empty() {
        return;
    }

    let mut total = 0;

    let task_pool = AsyncComputeTaskPool::get();

    for ev in ev_changed_coord.read() {
        for x in (ev.new_position.x - settings.world.world_dimension - settings.world.preload_extra_distance)..(ev.new_position.x + settings.world.world_dimension + settings.world.preload_extra_distance) {
            for y in (ev.new_position.y - settings.world.world_dimension - settings.world.preload_extra_distance)..(ev.new_position.y + settings.world.world_dimension + settings.world.preload_extra_distance) {
                let chunk_coord = ChunkCoord::new(x, y);

                if game_world.get(&chunk_coord).is_none() {
                    total += 1;
                    let task = task_pool.spawn(async move {
                        generate_chunk(&chunk_coord)
                    });

                    generation_tasks.chunks.insert(chunk_coord, task);
                };
            }
        }
    }

    if settings.logs.update_as_we_move_enabled {
        info!("Generated {} chunks", total);
    }
}

pub fn receive_generated_map_chunks(
    block_material: Res<BlockMaterial>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut generation_tasks: ResMut<ChunkGenerationTaskMap>
) {
    generation_tasks.chunks.retain(|chunk_coord, task| {
        let status = block_on(future::poll_once(task));

        let retain = status.is_none();

        if let Some(chunk_data) = status {
            spawn_chunk_from_data(chunk_data, *chunk_coord, &block_material, &mut mesh_manager, &mut commands);
        }

        retain
    });
}

pub fn touch_chunks_around_player_at_interval(
    mut query: Query<(&ChunkCoord, &mut ChunkKeepAlive, &mut Visibility)>,
    player_last_chunk_coord: Res<PlayerLastChunkCoord>,
    time: Res<Time>,
    settings: Res<Settings>
) {
    let mut total = 0;
    for (coord, mut keepalive, mut visibility) in query.iter_mut() {
        if (coord.x - player_last_chunk_coord.x).abs() < (settings.world.world_dimension + settings.world.preload_extra_distance) && (coord.y - player_last_chunk_coord.y).abs() < (settings.world.world_dimension + settings.world.preload_extra_distance) {
            keepalive.last_touch = time.elapsed_secs();

            total += 1;

            if (coord.x - player_last_chunk_coord.x).abs() < settings.world.world_dimension && (coord.y - player_last_chunk_coord.y).abs() < settings.world.world_dimension {
                *visibility = Visibility::Visible;
            }
        }
    }

    if total > 0 && settings.logs.update_as_we_move_enabled {
        info!("Touch chunks {} times", total);
    }
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct RemoveChunkTasks(HashMap<ChunkCoord, Task<Entity>>);

pub fn remove_chunks_that_are_stale(
    mut query: Query<(Entity, &ChunkCoord, &ChunkKeepAlive, &mut Visibility)>,
    time: Res<Time>,
    mut commands: Commands,
    mut game_world: ResMut<GameWorld>,
    settings: Res<Settings>
) {
    let mut total = 0;

    for (entity, chunk_coord, keepalive, mut visibility) in query.iter_mut() {
        if (time.elapsed_secs() - keepalive.last_touch).abs() > 3.0 {
            *visibility = Visibility::Hidden;
            
            if (time.elapsed_secs() - keepalive.last_touch).abs() > 6.0 {
                game_world.remove(chunk_coord);

                commands.entity(entity).despawn();

                total += 1;
            }
        }
    }

    if total > 0 && settings.logs.update_as_we_move_enabled {
        info!("Removed {} stale chunks", total);
    }
}

pub fn count_number_of_triangles_in_chunk_meshes(
    query: Query<&Mesh3d, With<ChunkCoord>>,
    meshes: Res<Assets<Mesh>>,
    settings: Res<Settings>
) {
    if !settings.logs.triangle_count_enabled {
        return;
    }

    let mut total = 0;

    for mesh in query.iter() {
        let t = meshes.get(&**mesh).unwrap();

        total += t.indices().unwrap().iter().count() / 3
    }

    info!("Total triangles: {}", total);
}