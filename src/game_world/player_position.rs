use crate::game_world::coord::ChunkCoord;
use crate::player::ThePlayer;
use crate::settings::{GameSettingResource, GameSettings, GameSettingsHandle};
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut, Debug, Clone, Default)]
pub struct PlayerLastChunkCoord(ChunkCoord);

#[derive(Event)]
pub struct PlayerChangedChunkCoordEvent {
    pub new_position: ChunkCoord,
    #[allow(dead_code)]
    pub previous_position: ChunkCoord,
}

pub fn check_for_player_chunk_position_update(
    player_last_chunk_coord: Res<PlayerLastChunkCoord>,
    player: Query<&Transform, With<ThePlayer>>,
    mut ev_changed_coord: EventWriter<PlayerChangedChunkCoordEvent>,
) {
    if let Ok(transform) = player.single() {
        let player_chunk_coord: ChunkCoord = ChunkCoord::from(*transform);

        if player_chunk_coord != player_last_chunk_coord.0 {
            ev_changed_coord.write(PlayerChangedChunkCoordEvent {
                new_position: player_chunk_coord,
                previous_position: player_last_chunk_coord.0,
            });
        }
    }
}

pub fn update_player_last_chunk_coord(
    mut player_last_chunk_coord: ResMut<PlayerLastChunkCoord>,
    mut ev_changed_coord: EventReader<PlayerChangedChunkCoordEvent>,
    game_setting_resource: Res<GameSettingResource>
) {
    if !game_setting_resource.settings.logs.change_chunk_enabled {
        return;
    }

    for ev in ev_changed_coord.read() {
        info!("Player is now in chunk {}", *ev.new_position);

        player_last_chunk_coord.0 = ev.new_position;
    }
}
