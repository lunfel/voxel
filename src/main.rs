use bevy::image::{ImageFilterMode, ImageSamplerDescriptor};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy::pbr::wireframe::WireframePlugin;
use bevy_rapier3d::prelude::*;

use player::PlayerPlugin;
use crate::logging::LoggingPlugin;
use crate::screen::ScreenPlugin;
use crate::selection::SelectionPlugin;
use crate::settings::Settings;
use crate::sun::SunPlugin;
use crate::world::world_generation::{BlockMaterial, BlockMaterialMap, WorldGenerationPlugin};
use crate::world::WorldPlugin;

mod screen;
mod settings;
mod utils;
mod world;
mod player;
mod logging;
mod selection;
mod sun;

fn main() {
    App::new()
        .init_resource::<Settings>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // This slows down the game by a lot
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(DefaultPlugins)
        // https://github.com/bevyengine/bevy/discussions/1289#discussioncomment-304058
        // https://github.com/bevyengine/bevy/issues/8846#issue-1757760152
        .add_plugins(
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    min_filter: ImageFilterMode::Nearest,
                    ..default()
                }
            })
        )
        .add_plugins(WorldGenerationPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SunPlugin)
        .add_plugins(ScreenPlugin)
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(WireframePlugin)
        .add_plugins(SelectionPlugin)
        .add_plugins(LoggingPlugin)
        .insert_resource(ClearColor(Color::srgba(0.4, 0.7, 0.85, 1.0)))
        // .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<BlockMaterialMap>()
        .init_resource::<BlockMaterial>()
        .run();
}
