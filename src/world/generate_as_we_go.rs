use crate::logging::LogIntervalTimer;
use crate::player::control::ThePlayer;
use crate::settings::{CoordSystemIntegerSize, CHUNK_SIZE, WORLD_DIMENSION};
use crate::world::world_generation::{generate_and_spawn_chunk, BlockMaterial, WorldGenerationState};
use bevy::prelude::*;
use bevy_rapier3d::na::Point2;
use crate::world::chunk::world_transform_to_chunk_coordinates;

#[derive(Resource, Deref, DerefMut, Debug, Clone, Default)]
pub struct PlayerLastChunkCoord(Point2<CoordSystemIntegerSize>);

#[derive(Event)]
pub struct PlayerChangedChunkCoordEvent {
    pub new_position: Point2<CoordSystemIntegerSize>,
    pub previous_position: Point2<CoordSystemIntegerSize>,
}

pub fn check_for_player_chunk_position_update(
    mut world_generation_state: ResMut<WorldGenerationState>,
    player_last_chunk_coord: Res<PlayerLastChunkCoord>,
    log_interval: Res<LogIntervalTimer>,
    player: Query<&Transform, With<ThePlayer>>,
    mut ev_changed_coord: EventWriter<PlayerChangedChunkCoordEvent>,
) {
    if let Ok(transform) = player.get_single() {
        let player_chunk_coord: Point2<CoordSystemIntegerSize> = world_transform_to_chunk_coordinates(transform);

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
) {
    for ev in ev_changed_coord.read() {
        info!("Player is now in chunk {}", ev.new_position);

        player_last_chunk_coord.0 = ev.new_position;
    }
}

pub fn spawn_and_destroy_chunks(
    mut world_generation_state: ResMut<WorldGenerationState>,
    block_material: Res<BlockMaterial>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut ev_changed_coord: EventReader<PlayerChangedChunkCoordEvent>,
) {
    for ev in ev_changed_coord.read() {
        // For X coords
        if ev.new_position.x + WORLD_DIMENSION > world_generation_state.generated_chunk_range_x.end {
            for x in world_generation_state.generated_chunk_range_x.end..ev.new_position.x + WORLD_DIMENSION {
                for z in world_generation_state.generated_chunk_range_z.clone() {
                    generate_and_spawn_chunk(&block_material, &mut mesh_manager, &mut commands, x, z);
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
                    generate_and_spawn_chunk(&block_material, &mut mesh_manager, &mut commands, x, z);
                }
            }
        }

        // For Z coords
        if ev.new_position.y + WORLD_DIMENSION > world_generation_state.generated_chunk_range_z.end {
            for z in world_generation_state.generated_chunk_range_z.end..ev.new_position.y + WORLD_DIMENSION {
                for x in world_generation_state.generated_chunk_range_x.clone() {
                    generate_and_spawn_chunk(&block_material, &mut mesh_manager, &mut commands, x, z);
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
                    generate_and_spawn_chunk(&block_material, &mut mesh_manager, &mut commands, x, z);
                }
            }
        }
    }
}