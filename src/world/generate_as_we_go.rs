use std::collections::HashMap;
use crate::logging::LogIntervalTimer;
use crate::player::control::ThePlayer;
use crate::settings::{CoordSystemIntegerSize, CHUNK_SIZE, WORLD_DIMENSION};
use crate::world::world_generation::{generate_chunk, spawn_chunk_from_data, BlockMaterial, ChunkData, WorldGenerationState};
use bevy::prelude::*;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use bevy_rapier3d::na::Point2;
use crate::world::chunk::{world_transform_to_chunk_coordinates, ChunkCoord};

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
    mut world_generation_state: ResMut<WorldGenerationState>,
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
                previous_position: player_last_chunk_coord.0.clone()
            });
        }
    }
}

pub fn update_player_last_chunk_coord(
    mut player_last_chunk_coord: ResMut<PlayerLastChunkCoord>,
    mut ev_changed_coord: EventReader<PlayerChangedChunkCoordEvent>,
) {
    for ev in ev_changed_coord.read() {
        info!("Player is now in chunk {}", *ev.new_position);

        player_last_chunk_coord.0 = ev.new_position.clone();
    }
}

pub fn begin_generating_map_chunks(
    mut world_generation_state: ResMut<WorldGenerationState>,
    mut ev_changed_coord: EventReader<PlayerChangedChunkCoordEvent>,
    mut generation_tasks: ResMut<ChunkGenerationTaskMap>
) {
    if ev_changed_coord.is_empty() {
        return;
    }

    let task_pool = AsyncComputeTaskPool::get();

    for ev in ev_changed_coord.read() {
        // For X coords
        if ev.new_position.x + WORLD_DIMENSION > world_generation_state.generated_chunk_range_x.end {
            for x in world_generation_state.generated_chunk_range_x.end..ev.new_position.x + WORLD_DIMENSION {
                for z in world_generation_state.generated_chunk_range_z.clone() {
                    let chunk_coord = ChunkCoord::new(x, z);
                    let chunk_coord_copy = chunk_coord.clone();

                    let task = task_pool.spawn(async move {
                        generate_chunk(&chunk_coord_copy)
                    });

                    generation_tasks.chunks.insert(chunk_coord, task);
                }
            }

            for x in world_generation_state.generated_chunk_range_x.start..ev.new_position.x - WORLD_DIMENSION {
                for z in world_generation_state.generated_chunk_range_z.clone() {
                    // todo! destroy corresponding chunks
                }
            }

            world_generation_state.generated_chunk_range_x = ev.new_position.x - WORLD_DIMENSION ..ev.new_position.x + WORLD_DIMENSION;
        } else if ev.new_position.x - WORLD_DIMENSION < world_generation_state.generated_chunk_range_x.start {
            for x in ev.new_position.x - WORLD_DIMENSION..world_generation_state.generated_chunk_range_x.start {
                for z in world_generation_state.generated_chunk_range_z.clone() {
                    let chunk_coord = ChunkCoord::new(x, z);
                    let chunk_coord_copy = chunk_coord.clone();

                    let task = task_pool.spawn(async move {
                        generate_chunk(&chunk_coord_copy)
                    });

                    generation_tasks.chunks.insert(chunk_coord, task);
                }
            }
        }

        // For Z coords
        if ev.new_position.y + WORLD_DIMENSION > world_generation_state.generated_chunk_range_z.end {
            for z in world_generation_state.generated_chunk_range_z.end..ev.new_position.y + WORLD_DIMENSION {
                for x in world_generation_state.generated_chunk_range_x.clone() {
                    let chunk_coord = ChunkCoord::new(x, z);
                    let chunk_coord_copy = chunk_coord.clone();

                    let task = task_pool.spawn(async move {
                        generate_chunk(&chunk_coord_copy)
                    });

                    generation_tasks.chunks.insert(chunk_coord, task);
                }
            }

            for z in world_generation_state.generated_chunk_range_z.start..ev.new_position.y - WORLD_DIMENSION {
                for x in world_generation_state.generated_chunk_range_x.clone() {
                    // todo! destroy corresponding chunks
                }
            }

            world_generation_state.generated_chunk_range_z = ev.new_position.y - WORLD_DIMENSION ..ev.new_position.y + WORLD_DIMENSION;
        } else if ev.new_position.y - WORLD_DIMENSION < world_generation_state.generated_chunk_range_z.start {
            for z in ev.new_position.y - WORLD_DIMENSION..world_generation_state.generated_chunk_range_z.start {
                for x in world_generation_state.generated_chunk_range_x.clone() {
                    let chunk_coord = ChunkCoord::new(x, z);
                    let chunk_coord_copy = chunk_coord.clone();

                    let task = task_pool.spawn(async move {
                        generate_chunk(&chunk_coord_copy)
                    });

                    generation_tasks.chunks.insert(chunk_coord, task);
                }
            }
        }
    }
}

pub fn receive_generated_map_chunks(
    block_material: Res<BlockMaterial>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut generation_tasks: ResMut<ChunkGenerationTaskMap>,
) {
    generation_tasks.chunks.retain(|chunk_coord, task| {
        let status = block_on(future::poll_once(task));

        let retain = status.is_none();

        if let Some(chunk_data) = status {
            spawn_chunk_from_data(chunk_data, chunk_coord.clone(), &block_material, &mut mesh_manager, &mut commands);
        }

        retain
    });
}