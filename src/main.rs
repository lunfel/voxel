mod screen;
mod settings;
mod systems;
mod utils;
mod world;

use crate::screen::ScreenPlugin;
use crate::world::WorldPlugin;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use systems::player::PlayerPlugin;
use crate::world::world_generation::{BlockMaterial, BlockMaterialMap, WorldGenerationPlugin};

fn main() {
    App::new()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // This slows down the game by a lot
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldGenerationPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScreenPlugin)
        .add_systems(Startup, setup_physics)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WorldPlugin)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.7, 0.85)))
        // .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<BlockMaterialMap>()
        .init_resource::<BlockMaterial>()
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
