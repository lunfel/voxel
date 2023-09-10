use noise::{Perlin, NoiseFn};

use crate::{resources::world::{GameChunk, CHUNK_SIZE, GameBlockType}, Point3D};

pub fn generate_single_chunk(coord: &Point3D) -> GameChunk {
    let perlin = Perlin::new(1);

    let mut game_chunk = GameChunk::new();

    for x in 0..CHUNK_SIZE {
       for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let px = (x as f64 / 20.0 + 0.1) + (coord.0 as f64 * CHUNK_SIZE as f64 / 20.0);
                let pz = (z as f64 / 20.0 + 0.1) + (coord.2 as f64 * CHUNK_SIZE as f64 / 20.0);

                let value = perlin.get([px, pz]) + 1.0;

                // info!("perlin: {}, height adj: {}", value, (value * 4.0).round() as u8 + 1);
                if ((value * 4.0).round()) as u8 + 1 > y as u8 {
                    game_chunk.blocks[x][y][z].block_type = GameBlockType::Ground
                }
            }
        } 
    } 

    game_chunk
}
