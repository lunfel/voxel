use crate::chunk::voxel_chunk::VoxelChunk;
use crate::game_world::coord::ChunkCoord;
use crate::game_world::GameWorld;
use crate::settings::{GameSettingResource, GameSettings, GameSettingsHandle, NoiseConfigurationChangedEvent};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use crate::FORM_VALUE_QUEUE;

pub struct TomlAssetPlugin;

impl Plugin for TomlAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GameSettings>()
            // .init_resource::<GameSettingResource>()
            .insert_resource(GameSettingResource::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (
                listen_to_settings_loaded,
                listen_to_noise_configuration_changed,
                debug_resource
            ))
            .register_asset_loader(TomlAssetLoader);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<GameSettings> = asset_server.load("game.toml");

    commands.insert_resource(GameSettingsHandle { handle });
}

fn listen_to_noise_configuration_changed(
    query: Query<Entity, With<VoxelChunk>>,
    mut game_world: ResMut<GameWorld>,
    mut commands: Commands,
    mut ev_changed_coord: EventWriter<crate::game_world::PlayerChangedChunkCoordEvent>,
    player_last_chunk_coord: Res<crate::game_world::PlayerLastChunkCoord>,
    mut event_reader: EventReader<NoiseConfigurationChangedEvent>,
) {
    for _ in event_reader.read() {
        info!("Noise configuration changed");

        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        game_world.clear();

        ev_changed_coord.write(crate::game_world::PlayerChangedChunkCoordEvent {
            new_position: ChunkCoord(player_last_chunk_coord.0),
            previous_position: ChunkCoord(player_last_chunk_coord.0),
        });
    }
}

fn debug_resource(
    mut game_setting_resource: ResMut<GameSettingResource>,
    mut events: EventWriter<NoiseConfigurationChangedEvent>,
)
{
    let mut queue = FORM_VALUE_QUEUE.lock().expect("Failed to lock queue");
    for val in queue.drain(..) {
        let base_noise = &mut game_setting_resource.settings.procedural.base_noise;
        base_noise.set_seed(val.seed);
        base_noise.amplitude = val.amplitude;
        base_noise.frequency = val.frequency;
        base_noise.gain = val.gain;
        base_noise.lacunarity = val.lacunarity;
        base_noise.octaves = val.octaves;

        events.write(NoiseConfigurationChangedEvent);
    }

    if game_setting_resource.is_added() {
        info!("Added GameSettingResource: {:?}", game_setting_resource);
    } else if game_setting_resource.is_changed() {
        info!("Changed GameSettingResource: {:?}", game_setting_resource);
    }
}

fn listen_to_settings_loaded(
    mut ev_asset: EventReader<AssetEvent<GameSettings>>,
    mut game_setting_resource: ResMut<GameSettingResource>,
    game_settings_handle: Res<GameSettingsHandle>,
    game_settings_assets: Res<Assets<GameSettings>>,
    mut event_writer: EventWriter<NoiseConfigurationChangedEvent>,
) {
    for ev in ev_asset.read() {
        info!("Processing asset event {:?}", ev);
        match ev {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => {
                if game_settings_handle.handle.id() == *id {
                    let game_settings = game_settings_assets.get(*id);

                    if let Some(game_settings) = game_settings {
                        *game_setting_resource = GameSettingResource {
                            settings: game_settings.clone(),
                        }
                    }

                    event_writer.write(NoiseConfigurationChangedEvent);
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
        info!("Loading toml asset");

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
