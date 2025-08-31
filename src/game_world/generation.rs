use crate::chunk::block::BlockMaterial;
use crate::chunk::chunk::{spawn_chunk_from_data, ChunkData};
use crate::chunk::procedural::generate_chunk;
use crate::game_world::coord::ChunkCoord;
use crate::game_world::player_position::{PlayerChangedChunkCoordEvent, PlayerLastChunkCoord};
use crate::game_world::GameWorld;
use crate::settings::{GameSettings, GameSettingsHandle};
use bevy::asset::ErasedAssetLoader;
use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use bevy::utils::HashMap;
use bevy_rapier3d::na::Point2;

#[derive(Resource, Default)]
pub struct WorldGenerationState {
    pub finished_generating: bool,
}

#[derive(Resource, Debug, Default)]
pub struct ChunkGenerationTaskMap {
    chunks: HashMap<ChunkCoord, Task<ChunkData>>,
}

#[derive(Component, Default, Debug)]
pub struct ChunkKeepAlive {
    // Stores the game Time.elapsed to compare staleness
    pub last_touch: f32,
}

pub fn begin_generating_map_chunks(
    mut ev_changed_coord: EventReader<PlayerChangedChunkCoordEvent>,
    mut generation_tasks: ResMut<ChunkGenerationTaskMap>,
    game_world: Res<GameWorld>,
    game_settings_handle: Res<GameSettingsHandle>,
    game_settings_assets: Res<Assets<GameSettings>>,
) {
    if ev_changed_coord.is_empty() {
        return;
    }

    let game_settings = game_settings_assets
        .get(&game_settings_handle.handle)
        .expect("This should have been loaded, but was not");

    info!(
        "Game settings from begin_generating_map_chunks: {:?}",
        game_settings
    );

    let mut total = 0;

    let task_pool = AsyncComputeTaskPool::get();

    for ev in ev_changed_coord.read() {
        for x in (ev.new_position.x
            - game_settings.world.world_dimension
            - game_settings.world.preload_extra_distance)
            ..(ev.new_position.x
                + game_settings.world.world_dimension
                + game_settings.world.preload_extra_distance)
        {
            for y in (ev.new_position.y
                - game_settings.world.world_dimension
                - game_settings.world.preload_extra_distance)
                ..(ev.new_position.y
                    + game_settings.world.world_dimension
                    + game_settings.world.preload_extra_distance)
            {
                let chunk_coord = ChunkCoord(Point2::new(x, y));

                if game_world.get(&chunk_coord).is_none() {
                    total += 1;

                    if game_settings.logs.update_as_we_move_enabled {
                        info!("{:?} has been updated", chunk_coord);
                    }

                    let gs = game_settings.clone();
                    let task = task_pool.spawn(async move { generate_chunk(&chunk_coord, &gs) });

                    generation_tasks.chunks.insert(chunk_coord, task);
                };
            }
        }
    }

    if game_settings.logs.update_as_we_move_enabled {
        info!("Generated {} chunks", total);
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
            spawn_chunk_from_data(
                chunk_data,
                *chunk_coord,
                &block_material,
                &mut mesh_manager,
                &mut commands,
            );
        }

        retain
    });
}

pub fn touch_chunks_around_player_at_interval(
    mut query: Query<(&ChunkCoord, &mut ChunkKeepAlive, &mut Visibility)>,
    player_last_chunk_coord: Res<PlayerLastChunkCoord>,
    time: Res<Time>,
    game_settings_handle: Res<GameSettingsHandle>,
    game_settings_assets: Res<Assets<GameSettings>>,
) {
    let mut total = 0;
    let game_settings = game_settings_assets
        .get(&game_settings_handle.handle)
        .expect("This should have been loaded, but was not");

    for (coord, mut keepalive, mut visibility) in query.iter_mut() {
        if (coord.x - player_last_chunk_coord.x).abs()
            < (game_settings.world.world_dimension + game_settings.world.preload_extra_distance)
            && (coord.y - player_last_chunk_coord.y).abs()
                < (game_settings.world.world_dimension + game_settings.world.preload_extra_distance)
        {
            keepalive.last_touch = time.elapsed_secs();

            total += 1;

            if (coord.x - player_last_chunk_coord.x).abs() < game_settings.world.world_dimension
                && (coord.y - player_last_chunk_coord.y).abs() < game_settings.world.world_dimension
            {
                *visibility = Visibility::Visible;
            }
        }
    }

    if total > 0 && game_settings.logs.update_as_we_move_enabled {
        info!("Touch chunks {} times", total);
    }
}
