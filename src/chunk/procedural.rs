use crate::settings::{CoordSystemIntegerSize, CHUNK_HEIGHT, CHUNK_SIZE};
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy_rapier3d::na::Point3;
use bevy_rapier3d::prelude::Vect;
use crate::chunk::block::{VoxelBlock, VoxelBlockType};
use crate::chunk::chunk::{ChunkData, VoxelChunk};
use crate::chunk::perlin::{PerlinCoord, PerlinCoord3d};
use noise::{NoiseFn, Perlin};
use crate::game_world::coord::{ChunkCoord, LocalVoxelBlockCoord};
use crate::utils::render_mesh;

pub fn generate_chunk(chunk_coord: &ChunkCoord) -> ChunkData {
    let chunk = generate_single_chunk(chunk_coord);

    let (indices, vertices) =  chunk.render_indices_and_vertices();

    // let mesh = Mesh3d(mesh_manager.add(render_mesh(&indices, &vertices)));
    let mesh = render_mesh(&indices, &vertices);

    // let mut block_transforms = vec![];
    //
    // for x in 0..CHUNK_SIZE {
    //     for y in 0..CHUNK_HEIGHT {
    //         for z in 0..CHUNK_SIZE {
    //             if let Some(block) = chunk.get_block(&LocalVoxelBlockCoord(Point3::new(x, y, z))) {
    //                 if !block.is_fully_surrounded && block.block_type != VoxelBlockType::Empty {
    //                     let block_transform = Transform::from_xyz(
    //                         (chunk_coord.x * CHUNK_SIZE + x) as f32,
    //                         (CHUNK_HEIGHT + y) as f32,
    //                         (chunk_coord.y * CHUNK_SIZE + z) as f32
    //                     );
    //
    //                     block_transforms.push(block_transform);
    //                     //commands.spawn(block_transform);
    //                 }
    //             }
    //         }
    //     }
    // }

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


    ChunkData {
        mesh,
        vertex: v,
        indices: i,
        chunk
    }
}

pub fn generate_single_chunk<P>(coord: &P) -> VoxelChunk
where P: Into<ChunkCoord> + Clone
{
    let _span = info_span!("generate_single_chunk").entered();

    let continentality_perlin = Perlin::new(10);
    let continentality_frequency = 1.0 / 180.0;
    let continentality_amplitude = 1.0;

    let block_perlin = Perlin::new(20);

    let height_perlin = Perlin::new(1);
    let frequency1 = 1.0 / 40.0;
    let amplitude1 = 50.0;

    let height_perlin2 = Perlin::new(3);
    let frequency2 = 1.0 / 15.0;
    let amplitude2 = 25.0;

    // Should be between 0.0 and 1.0 (it's a percentage)
    let height_persistence: f64 = 0.9;
    let height_lacunarity: f64 = 2.5;
    let height_base_amplitude: f64 = 40.0;
    let height_base_frequency: f64 = 1.0 / 120.0;

    let height_perlin_new = Perlin::new(1);

    let ground_layer_perlin = Perlin::new(2);
    let chunk_coord: ChunkCoord = (*coord).clone().into();

    let mut game_chunk = VoxelChunk::default();
    let offset = 0.1153;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let block_coord = LocalVoxelBlockCoord(Point3::new(x, y, z));

                let perlin_coord = PerlinCoord::from_voxel_block_coord_with_offset(
                    block_coord,
                    chunk_coord,
                    offset,
                );

                let perlin_coord3d = PerlinCoord3d::from_voxel_block_coord_with_offset(
                    block_coord,
                    chunk_coord,
                    offset
                );

                // perlin.get gives an f64 value between -1 and 1
                // let continentality_value = ((continentality_perlin.get(perlin_coord * continentality_frequency) + 1.0) / 2.0) * continentality_amplitude;
                // let height_value = ((height_perlin.get(perlin_coord * frequency1) + 1.0) / 2.0) * (amplitude1 * (continentality_value + 0.1));
                // let height_value2 = ((height_perlin2.get(perlin_coord * frequency2) + 1.0) / 2.0) * amplitude2 * (continentality_value + 0.1);
                // let height = (height_value + height_value2) as usize;

                // New version of height map
                let mut height: CoordSystemIntegerSize = 0;
                for octave in 1..3 {
                    let frequency = height_lacunarity.powf(octave as f64) * height_base_frequency;
                    let amplitude = height_persistence.powf(octave as f64) * height_base_amplitude;
                    let octave_height = (((height_perlin_new.get(perlin_coord * frequency) + 1.0) / 2.0) * amplitude) as CoordSystemIntegerSize;

                    height += octave_height;
                }

                let block_value = (block_perlin.get(perlin_coord3d * (1.0 / 40.0)) + 1.0) / 2.0;

                // info!("perlin value {}", block_value);

                if height >= y {
                    if let Some(block) = game_chunk.get_block_mut(&block_coord) {
                        block.block_type = match block_value {
                            0.40..0.41 => VoxelBlockType::Gem,
                            0.41..0.60 => VoxelBlockType::Rock,
                            // 0.60..0.68 => GameBlockType::Empty,
                            0.68..0.70 => VoxelBlockType::Dirt,
                            0.70..1.0 => VoxelBlockType::Grass,
                            _ => VoxelBlockType::Dirt,
                        }
                    }
                }

                // if height == y {
                //     game_chunk.blocks[x][y][z].block_type = if ground_layer_perlin.get(perlin_coord.0) > 0.5 {
                //         GameBlockType::Rock
                //     } else if ground_layer_perlin.get(perlin_coord.0) > 0.4 {
                //         GameBlockType::Gem
                //     } else {
                //         GameBlockType::Ground
                //     }
                // } else if height > y {
                //     game_chunk.blocks[x][y][z].block_type = GameBlockType::Rock
                // }
            }
        }
    }

    game_chunk
}