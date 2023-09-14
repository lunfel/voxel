use std::cmp::{max, min};
use bevy::prelude::*;
use bevy_inspector_egui::egui::lerp;
use noise::{Perlin, NoiseFn};

use crate::{world::{chunk::GameChunk, block::GameBlockType, GameWorld}, utils::point::Point3D, settings::CHUNK_SIZE};

#[derive(Resource)]
pub struct WorldGenerationState {
    pub finished_generating: bool
}

impl Default for WorldGenerationState {
    fn default() -> Self {
        Self {
            finished_generating: false
        }
    }
}

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        info!("WorldGenerationPlugin");
        app
            .init_resource::<WorldGenerationState>()
            .add_systems(Startup, generate_world);
    }
}

pub fn generate_single_chunk<P>(coord: &P) -> GameChunk
where P: Into<Point3D<usize>> + Clone
{
    let height_perlin = Perlin::new(1);
    let ground_layer_perlin = Perlin::new(2);
    let coord: Point3D<usize> = (*coord).clone().into();

    let mut game_chunk = GameChunk::new();

    for x in 0..CHUNK_SIZE {
       for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let px = (x as f64 / 20.0 + 0.1) + (coord.x as f64 * CHUNK_SIZE as f64 / 20.0);
                let pz = (z as f64 / 20.0 + 0.1) + (coord.z as f64 * CHUNK_SIZE as f64 / 20.0);

                // perlin.get gives an f64 value between -1 and 1
                let height_value = height_perlin.get([px, pz]) + 1.0;
                let height = (height_value * 6.0).round() as usize + 1;

                if height == y {
                    game_chunk.blocks[x][y][z].block_type = match ground_layer_perlin.get([px, pz]) {
                        0.5..=1.0 => GameBlockType::Rock,
                        _ => GameBlockType::Ground
                    }
                } else if height > y {
                    game_chunk.blocks[x][y][z].block_type = GameBlockType::Rock
                }
            }
        } 
    }

    game_chunk
}

pub fn generate_world(
    mut game_world: ResMut<GameWorld>,
    mut world_generation_state: ResMut<WorldGenerationState>
) {
    info!("Generate world chunks");
    for x in 0..8 {
        for z in 0..8 {
            let point: Point3D<usize> = (x as usize, 0 as usize, z as usize).into();
            let chunk = generate_single_chunk(&point);

            game_world.chunks.insert(point, chunk);
        }
    }

    world_generation_state.finished_generating = true;
}
