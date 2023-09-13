mod systems;
mod utils;
mod world;
mod settings;

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use systems::{world_generation::WorldGenerationPlugin, player::PlayerPlugin};
use bevy_rapier3d::prelude::*;
use world::GameWorld;


fn main() {
    App::new()
        .init_resource::<GameWorld>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldGenerationPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_physics)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
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

