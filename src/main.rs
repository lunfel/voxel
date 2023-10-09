mod systems;
mod utils;
mod world;
mod settings;
mod screen;

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use systems::{world_generation::WorldGenerationPlugin, player::PlayerPlugin};
use bevy_rapier3d::prelude::*;
use crate::screen::ScreenPlugin;
use crate::settings::GameParameters;
use crate::systems::world_generation::BlockMaterialMap;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .init_resource::<GameParameters>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldGenerationPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScreenPlugin)
        .add_systems(Startup, setup_physics)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WorldPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<BlockMaterialMap>()
        .run();
} 

fn setup_physics(mut commands: Commands) {
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.1))
        .insert(TransformBundle::from(Transform::from_xyz(16.0, 20.0, 16.0)));
}

