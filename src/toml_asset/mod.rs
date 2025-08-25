use crate::settings::{GameSettings, GameSettingsHandle};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;

pub struct TomlAssetPlugin;

impl Plugin for TomlAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GameSettings>()
            .add_systems(Startup, setup)
            .register_asset_loader(TomlAssetLoader);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<GameSettings> = asset_server.load("config.toml");

    commands.insert_resource(GameSettingsHandle(handle));
}

fn listen_to_settings_loaded(
    mut ev_asset: EventReader<AssetEvent<GameSettings>>,
    mut assets: ResMut<Assets<GameSettings>>,
    game_handle_resource: Res<GameSettingsHandle>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                // let settings = assets.get_mut(id).unwrap();

                if *game_handle_resource == id {
                    // it is our special map image!
                } else {
                    // it is some other image
                }
            }
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