use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Quat;
use bevy::pbr::{MeshMaterial3d, PointLight, StandardMaterial};
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy_rapier3d::math::Vect;
use bevy_rapier3d::na::Point2;
use crate::chunk::block::{BlockMaterial, VoxelBlockType};
use crate::chunk::chunk::{spawn_chunk_from_data, ChunkData, VoxelChunk};
use crate::game_world::coord::{ChunkCoord, LocalVoxelBlockCoord, LocalVoxelBlockOffset};
use crate::utils::render_mesh;

pub fn tmp_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    block_material: Res<BlockMaterial>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(mesh_manager.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(mesh_manager.add(Cuboid::new(1.0, 1.0, 1.0))),
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

    let chunk = generate_single_demo_chunk();

    let (indices, vertices) = chunk.render_indices_and_vertices();

    let mesh = render_mesh(&indices, &vertices);

    let v: Vec<Vect> = vertices.iter().map(|(v, _, _)| Vec3::from_array(*v)).collect();
    let i: Vec<[u32; 3]> = match indices {
        Indices::U16(_) => unimplemented!("Not used by the game"),
        Indices::U32(indices) => {
            indices.chunks(3)
                .map(|chunk| {
                    let mut vec: [u32; 3] = [0, 0, 0];

                    vec[0..3].clone_from_slice(&chunk[0..3]);

                    vec
                })
                .collect()
        }
    };

    let chunk_data = ChunkData {
        mesh,
        vertex: v,
        indices: i,
        chunk
    };

    spawn_chunk_from_data(chunk_data, ChunkCoord(Point2::new(0, 0)), &block_material, &mut mesh_manager, &mut commands);
}

fn generate_single_demo_chunk() -> VoxelChunk {
    let mut chunk = VoxelChunk::default();

    for (coord, block) in chunk.0.iter_mut()
        .enumerate()
        .map(|(idx, block)| (LocalVoxelBlockCoord::from(LocalVoxelBlockOffset(idx)), block))
    {
        if coord.y < 8 && coord.y > 4 {
            block.block_type = VoxelBlockType::Grass
        }
    }

    chunk
}