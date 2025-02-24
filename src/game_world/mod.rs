mod coord;

use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::game_world::coord::{ChunkCoord};

/// Holds the currently loaded chunks of the game world
pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tmp_setup);
    }
}

fn tmp_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

#[derive(Resource, Deref, DerefMut, Default)]
/// Entity is meant for VoxelChunk in this resource
pub struct GameWorld(pub HashMap<ChunkCoord, Entity>);

#[derive(Component)]
#[require(Transform)]
pub struct WillMakeChunkLoad;