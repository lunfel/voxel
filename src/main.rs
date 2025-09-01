mod chunk;
mod game_world;
mod logging;
mod player;
mod screen;
mod settings;
mod sun;
mod toml_asset;
mod utils;
mod web;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::image::{ImageFilterMode, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use wasm_bindgen::prelude::*;
use crate::chunk::ChunkPlugin;
use crate::logging::LoggingPlugin;
use crate::player::PlayerPlugin;
use crate::screen::ScreenPlugin;
use crate::sun::SunPlugin;
use crate::toml_asset::TomlAssetPlugin;
pub use game_world::GameWorldPlugin;
use crate::settings::{GameSettingResource, NoiseConfigurationChangedEvent};
use crate::web::setup_pointer_lock;
use std::cell::RefCell;
use std::sync::{LazyLock, Mutex};

static FORM_VALUE_QUEUE: LazyLock<Mutex<Vec<FormValue>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub struct FormValue {
    seed: u32,
    octaves: i32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    gain: f64,
}

#[wasm_bindgen]
pub fn set_form_value(
    seed: u32,
    octaves: i32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    gain: f64,
) {
    FORM_VALUE_QUEUE.lock().expect("Could not get lock on form value queue").push(FormValue {
        seed,
        octaves,
        frequency,
        amplitude,
        lacunarity,
        gain,
    });
}

fn main() {
    let mut app = App::new();

    let window = if cfg!(target_arch = "wasm32") {
        Window {
            canvas: Some("#bevy-canvas".to_string()),
            ..default()
        }
    } else {
        default()
    };

    // .init_resource::<Settings>()
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        min_filter: ImageFilterMode::Nearest,
                        ..default()
                    },
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
        )
        .add_systems(Startup, setup_pointer_lock)
        .insert_resource(ClearColor(Color::srgba(0.4, 0.7, 0.85, 1.0)))
        .add_plugins((
            TomlAssetPlugin,
            LoggingPlugin,
            GameWorldPlugin,
            PlayerPlugin,
            ChunkPlugin,
            ScreenPlugin,
            SunPlugin,
        ))
        .add_event::<NoiseConfigurationChangedEvent>()
        // Debug plugins
        // This slows down the game by a lot
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(DefaultPlugins)
        // https://github.com/bevyengine/bevy/discussions/1289#discussioncomment-304058
        // https://github.com/bevyengine/bevy/issues/8846#issue-1757760152
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin {
        //     max_history_length: 10,
        //     smoothing_factor: 0.2,
        // })
        // .add_plugins(WireframePlugin)
        // .insert_resource(WireframeConfig {
        //     global: true, // Toggle this to false to disable globally
        //     ..Default::default()
        // })
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}
