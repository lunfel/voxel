mod player;
mod game_world;
mod chunk;
mod settings;

use bevy::image::{ImageFilterMode, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub use game_world::GameWorldPlugin;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
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
            GameWorldPlugin,
            PlayerPlugin
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
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}
