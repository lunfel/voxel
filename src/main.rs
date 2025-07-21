mod player;
mod game_world;
mod chunk;
mod settings;
mod utils;
mod logging;

use bevy::image::{ImageFilterMode, ImageSamplerDescriptor};
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub use game_world::GameWorldPlugin;
use crate::chunk::ChunkPlugin;
use crate::logging::LoggingPlugin;
use crate::player::PlayerPlugin;
use crate::settings::Settings;

fn main() {
    App::new()
        .init_resource::<Settings>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    min_filter: ImageFilterMode::Nearest,
                    ..default()
                }
            })
        )
        .insert_resource(ClearColor(Color::srgba(0.4, 0.7, 0.85, 1.0)))
        .add_plugins((
            LoggingPlugin,
            GameWorldPlugin,
            PlayerPlugin,
            ChunkPlugin
        ))
        // Debug plugins
        // This slows down the game by a lot
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(DefaultPlugins)
        // https://github.com/bevyengine/bevy/discussions/1289#discussioncomment-304058
        // https://github.com/bevyengine/bevy/issues/8846#issue-1757760152
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin)
        // .add_plugins(WireframePlugin)
        // .insert_resource(WireframeConfig {
        //     global: true, // Toggle this to false to disable globally
        //     ..Default::default()
        // })
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}
