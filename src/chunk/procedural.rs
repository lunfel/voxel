use crate::settings::{CoordSystemIntegerSize, GameSettings, CHUNK_HEIGHT, CHUNK_SIZE};
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy_rapier3d::na::Point3;
use bevy_rapier3d::prelude::Vect;
use crate::chunk::block::{VoxelBlock, VoxelBlockType};
use crate::chunk::chunk::{ChunkData, VoxelChunk};
use crate::chunk::perlin::{PerlinCoord, PerlinCoord3d};
use noise::{NoiseFn, Perlin, Simplex};
use crate::chunk::noise::Noise;
use crate::game_world::coord::{ChunkCoord, LocalVoxelBlockCoord};
use crate::utils::render_mesh;

pub fn generate_chunk(chunk_coord: &ChunkCoord, game_settings: &GameSettings) -> ChunkData {
    let chunk = generate_single_chunk(chunk_coord, game_settings);

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

pub fn generate_single_chunk<P>(coord: &P, game_settings: &GameSettings) -> VoxelChunk
where P: Into<ChunkCoord> + Clone
{
    let _span = info_span!("generate_single_chunk").entered();

    let block_noise = &game_settings.procedural.block_noise;
    let height_noise = &game_settings.procedural.base_noise;

    let chunk_coord: ChunkCoord = (*coord).clone().into();

    let mut game_chunk = VoxelChunk::default();
    let offset = 0.1153;

    let mut min_value: f64 = 1000.0;
    let mut max_value: f64 = -1000.0;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                let block_coord = LocalVoxelBlockCoord(Point3::new(x, y, z));

                // https://www.reddit.com/r/proceduralgeneration/comments/6eubj7/how_can_i_add_octaves_persistence_lacunarity/
                // New version of height map
                let height: CoordSystemIntegerSize = height_noise.get([
                    (block_coord.x as f64) + (chunk_coord.x as f64 * CHUNK_SIZE as f64),
                    (block_coord.y as f64),
                    (block_coord.z as f64) + (chunk_coord.y as f64 * CHUNK_SIZE as f64)
                ]) as CoordSystemIntegerSize;

                let block_value = block_noise.get(
                    [
                        (block_coord.x as f64) + (chunk_coord.x as f64 * CHUNK_SIZE as f64),
                        (block_coord.y as f64),
                        (block_coord.z as f64) + (chunk_coord.y as f64 * CHUNK_SIZE as f64)
                    ]
                );

                min_value = min_value.min(block_value);
                max_value = max_value.max(block_value);

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

    // info!("perlin min: {}, max: {}", min_value, max_value);

    game_chunk
}