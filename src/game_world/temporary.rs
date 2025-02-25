use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Quat;
use bevy::pbr::{MeshMaterial3d, PointLight, StandardMaterial};
use bevy::prelude::{default, Circle, Commands, Cuboid, Mesh, Mesh3d, ResMut, Transform};
use crate::chunk::block::VoxelBlockType;
use crate::chunk::chunk::VoxelChunk;
use crate::game_world::coord::{LocalVoxelBlockCoord, LocalVoxelBlockOffset};

pub fn tmp_setup(
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

fn generate_single_demo_chunk() {
    let mut chunk = VoxelChunk::default();

    for (coord, block) in chunk.0.iter_mut()
        .enumerate()
        .map(|(idx, block)| (LocalVoxelBlockCoord::from(LocalVoxelBlockOffset(idx)), block))
    {
        if coord.y < 6 {
            block.block_type = VoxelBlockType::Grass
        }
    }
}