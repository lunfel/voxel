use crate::game_world::coord::ChunkCoord;
use crate::game_world::generation::WorldGenerationState;
use crate::logging::LogIntervalTimer;
use crate::player::ThePlayer;
use crate::settings::{GameSettings, GameSettingsHandle};
use bevy::prelude::*;

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
        let player_chunk_coord: ChunkCoord = ChunkCoord::from(*transform);

        if player_chunk_coord != player_last_chunk_coord.0 {
            ev_changed_coord.send(PlayerChangedChunkCoordEvent {
                new_position: player_chunk_coord,
                previous_position: player_last_chunk_coord.0,
            });
        }
    }
}

pub fn update_player_last_chunk_coord(
    mut player_last_chunk_coord: ResMut<PlayerLastChunkCoord>,
    mut ev_changed_coord: EventReader<PlayerChangedChunkCoordEvent>,
    game_settings_assets: Res<Assets<GameSettings>>,
    game_handle_resource: Res<GameSettingsHandle>,
) {
    let game_settings = game_settings_assets
        .get(&game_handle_resource.handle)
        .expect("This should have been loaded, but was not");

    if !game_settings.logs.change_chunk_enabled {
        return;
    }

    for ev in ev_changed_coord.read() {
        info!("Player is now in chunk {}", *ev.new_position);

        player_last_chunk_coord.0 = ev.new_position;
    }
}
