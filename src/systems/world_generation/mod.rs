use bevy::prelude::info;
use noise::{Perlin, NoiseFn, Seedable};

use crate::resources::world::{GameChunk, CHUNK_SIZE, GameBlockType};

pub fn generate_single_chunk() -> GameChunk {
    let perlin = Perlin::new(1);

    let mut game_chunk = GameChunk::new();

    for x in 0..CHUNK_SIZE {
       for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let value = perlin.get([x as f64 / 10.0 + 0.1, z as f64 / 10.0 + 0.1]) + 1.0;

                info!("perlin: {}, height adj: {}", value, (value * 4.0).round() as u8 + 1);
                if ((value * 4.0).round()) as u8 + 1 > y as u8 {
                    game_chunk.blocks[x][y][z].block_type = GameBlockType::Ground
                }
            }
        } 
    } 

    game_chunk
}
