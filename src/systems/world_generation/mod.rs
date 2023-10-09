use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use noise::{NoiseFn, Perlin};
use bevy_rapier3d::prelude::*;

use crate::{settings::CHUNK_SIZE, world::{block::GameBlockType, chunk::GameChunk}};
use crate::settings::GameParameters;
use crate::world::block::BlockCoord;
use crate::world::chunk::ChunkCoord;
use crate::world::systems::chunk::render_chunk;

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
        info!("WorldGenerationPlugin initializing");
        app
            .init_resource::<WorldGenerationState>()
            .add_systems(Startup, generate_world);
        info!("WorldGenerationPlugin loaded");
    }
}

pub fn generate_single_chunk<'a, P>(coord: &P) -> GameChunk
where P: Into<ChunkCoord> + Clone
{
    let height_perlin = Perlin::new(1);
    let ground_layer_perlin = Perlin::new(2);
    let coord: ChunkCoord = (*coord).clone().into();

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
                    game_chunk.blocks[x][y][z].block_type = if ground_layer_perlin.get([px, pz]) > 0.5 {
                        GameBlockType::Rock
                    } else {
                        GameBlockType::Ground
                    }
                } else if height > y {
                    game_chunk.blocks[x][y][z].block_type = GameBlockType::Rock
                }
            }
        } 
    }

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let block_coord: BlockCoord = (x, y, z).into();
                let mut is_fully_surrounded = true;
                for maybe_neighbor_coord in block_coord.neighbors() {
                    if let Some(neighbor_coord) = maybe_neighbor_coord {
                        if let Some (neighbor_block) = game_chunk.get_block(&neighbor_coord) {
                            if neighbor_block.block_type == GameBlockType::Empty {
                                is_fully_surrounded = false;
                                break;
                            }
                        } else {
                            is_fully_surrounded = false;
                        }
                    } else {
                        is_fully_surrounded = false;
                    }
                }

                game_chunk.update_block(&block_coord, |b| {
                    b.is_fully_surrounded = is_fully_surrounded;
                });
            }
        }
    }

    game_chunk
}

pub type BlockMaterialHashMap = HashMap<GameBlockType, Handle<StandardMaterial>>;

#[derive(Resource, Deref, DerefMut)]
pub struct BlockMaterialMap(BlockMaterialHashMap);

impl FromWorld for BlockMaterialMap {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let mut material_map: BlockMaterialHashMap = HashMap::new();

        material_map.insert(GameBlockType::Rock, materials.add(Color::rgb(79.0 / 255.0, 87.0 / 255.0, 99.0 / 255.0).into()));
        material_map.insert(GameBlockType::Ground, materials.add(Color::rgb(76.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0).into()));

        Self(material_map)
    }
}

pub fn generate_world(
    mut world_generation_state: ResMut<WorldGenerationState>,
    block_material_map: Res<BlockMaterialMap>,
    game_parameters: Res<GameParameters>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
    mut commands: Commands
) {
    info!("Generate world chunks");

    let dimension = 4;

    for x in 0..dimension {
        for z in 0..dimension {
            let chunk_coord: ChunkCoord = (x as usize, 0 as usize, z as usize).into();

            let transform = Transform::from_xyz(
                (chunk_coord.x * game_parameters.chunk_size) as f32,
                (chunk_coord.y * game_parameters.chunk_size) as f32,
                (chunk_coord.z * game_parameters.chunk_size) as f32
            );

            info!("Spawning chunks");

            let chunk = generate_single_chunk(&chunk_coord);

            let mesh = render_chunk(&game_parameters, &chunk);

            let mesh_handle = mesh_manager.add(mesh);

            for x in 0..game_parameters.chunk_size {
                for y in 0..game_parameters.chunk_size {
                    for z in 0..game_parameters.chunk_size {
                        if let Some(block) = chunk.get_block(&(x, y, z)) {
                            if !block.is_fully_surrounded && block.block_type != GameBlockType::Empty {
                                let block_transform = Transform::from_xyz(
                                    (chunk_coord.x * game_parameters.chunk_size + x) as f32,
                                    (chunk_coord.y * game_parameters.chunk_size + y) as f32,
                                    (chunk_coord.z * game_parameters.chunk_size + z) as f32
                                );

                                commands.spawn((
                                    block_transform,
                                    Collider::cuboid(0.5, 0.5, 0.5),
                                    RigidBody::Fixed,
                                    GlobalTransform::default(),
                                    Friction {
                                        coefficient: 0.0,
                                        combine_rule: CoefficientCombineRule::Min
                                    }
                                ));
                            }
                        }
                    }
                }
            }

            commands.spawn((PbrBundle {
                transform,
                mesh: mesh_handle,
                material: block_material_map.get(&GameBlockType::Ground).unwrap().clone(),
                ..default()
            }, chunk, chunk_coord));
        }
    }

    world_generation_state.finished_generating = true;
}