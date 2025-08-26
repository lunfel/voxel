use crate::settings::{GameSettings, GameSettingsHandle};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use crate::chunk::chunk::VoxelChunk;
use crate::game_world::coord::ChunkCoord;
use crate::game_world::GameWorld;

pub struct TomlAssetPlugin;

impl Plugin for TomlAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GameSettings>()
            .add_systems(Startup, setup)
            .register_asset_loader(TomlAssetLoader);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<GameSettings> = asset_server.load("game.toml");

    commands.insert_resource(GameSettingsHandle {
        handle
    });
}

fn listen_to_settings_loaded(
    mut ev_asset: EventReader<AssetEvent<GameSettings>>,
    mut assets: ResMut<Assets<GameSettings>>,
    game_settings_handle: Res<GameSettingsHandle>,
    query: Query<Entity, With<VoxelChunk>>,
    mut game_world: ResMut<GameWorld>,
    mut commands: Commands,
    mut ev_changed_coord: EventWriter<crate::game_world::PlayerChangedChunkCoordEvent>,
    player_last_chunk_coord: Res<crate::game_world::PlayerLastChunkCoord>
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                // let settings = assets.get_mut(id).unwrap();

                let game_settings = assets.get(&game_settings_handle.handle).expect("This should have been loaded, but was not");

                if game_settings_handle.handle.id() == *id {
                    for entity in query.iter() {
                        commands.entity(entity)
                            .despawn();
                    }

                    game_world.clear();

                    ev_changed_coord.send(crate::game_world::PlayerChangedChunkCoordEvent {
                        new_position: ChunkCoord(player_last_chunk_coord.0),
                        previous_position: ChunkCoord(player_last_chunk_coord.0)
                    });
                }
            }
            _ => {}
        }
    }
}

pub struct TomlAssetLoader;

impl AssetLoader for TomlAssetLoader {
    type Asset = GameSettings;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<GameSettings, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let text = std::str::from_utf8(&bytes)?;

        let config: GameSettings = toml::from_str(text)?;

        Ok(config)
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}