use std::ops::Mul;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::utils::hashbrown::HashMap;
use noise::{NoiseFn, Perlin};
use bevy_rapier3d::prelude::*;

use crate::{settings::CHUNK_SIZE, world::{block::GameBlockType, chunk::GameChunk}};
use crate::settings::{CHUNK_HEIGHT, CoordSystemIntegerSize};
use crate::utils::fresh_entity::FreshEntity;
use crate::utils::point::Point3D;
use crate::world::block::BlockCoord;
use crate::world::chunk::{ChunkCoord, Vertex};
use crate::world::systems::chunk::{render_indices_and_vertices, render_mesh};

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

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct PerlinCoord([f64; 2]);

impl Mul<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn mul(self, rhs: f64) -> Self::Output {
        return [
            &self[0] * rhs,
            &self[1] * rhs
        ]
    }
}

pub fn generate_single_chunk<P>(coord: &P) -> GameChunk
where P: Into<ChunkCoord> + Clone
{
    let continentality_perlin = Perlin::new(10);
    let continentality_frequency = 1.0 / 160.0;
    let continentality_amplitude = 1.0;

    let height_perlin = Perlin::new(1);
    let frequency1 = 1.0 / 40.0;
    let amplitude1 = 30.0;

    let height_perlin2 = Perlin::new(3);
    let frequency2 = 1.0 / 15.0;
    let amplitude2 = 15.0;

    let ground_layer_perlin = Perlin::new(2);
    let coord: ChunkCoord = (*coord).clone().into();

    let mut game_chunk = GameChunk::new();

    for x in 0..(CHUNK_SIZE as usize) {
       for y in 0..(CHUNK_HEIGHT as usize) {
            for z in 0..(CHUNK_SIZE as usize) {
                let perlin_coord = PerlinCoord([
                    (x as f64  + 0.1) + (coord.x as f64 * CHUNK_SIZE as f64),
                    (z as f64  + 0.1) + (coord.z as f64 * CHUNK_SIZE as f64)
                ]);

                // perlin.get gives an f64 value between -1 and 1
                let continentality_value = ((continentality_perlin.get(perlin_coord * continentality_frequency) + 1.0) / 2.0) * continentality_amplitude;
                let height_value = ((height_perlin.get(perlin_coord * frequency1) + 1.0) / 2.0) * (amplitude1 * (continentality_value + 0.1));
                let height_value2 = ((height_perlin2.get(perlin_coord * frequency2) + 1.0) / 2.0) * amplitude2 * (continentality_value + 0.1);
                let height = (height_value + height_value2) as usize;

                if height == (y as usize) {
                    game_chunk.blocks[x][y][z].block_type = if ground_layer_perlin.get(perlin_coord.0) > 0.5 {
                        GameBlockType::Rock
                    } else if ground_layer_perlin.get(perlin_coord.0) > 0.4 {
                        GameBlockType::Gem
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

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct BlockMaterial(Handle<StandardMaterial>);

impl FromWorld for BlockMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut asset_server = world.resource_mut::<AssetServer>();
        let handle_image = asset_server.load("block-textures.png");

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let handle_material = materials.add(handle_image);

        Self(handle_material)
    }
}

impl FromWorld for BlockMaterialMap {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let mut material_map: BlockMaterialHashMap = HashMap::new();

        material_map.insert(GameBlockType::Rock, materials.add(Color::rgb(79.0 / 255.0, 87.0 / 255.0, 99.0 / 255.0)));
        material_map.insert(GameBlockType::Ground, materials.add(Color::rgb(76.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0)));

        Self(material_map)
    }
}

pub fn generate_world(
    mut world_generation_state: ResMut<WorldGenerationState>,
    block_material: Res<BlockMaterial>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
    mut commands: Commands
) {
    info!("Generate world chunks");

    // Let's assume player is at 0,0,0 for now

    let player_position: Point3D<i32> = Point3D::default();
    let dimension = 16;

    for x in player_position.x - dimension..player_position.x + dimension {
        for z in player_position.z - dimension..player_position.z + dimension {
            let chunk_coord: ChunkCoord = (x as CoordSystemIntegerSize, 0 as CoordSystemIntegerSize, z as CoordSystemIntegerSize).into();

            let chunk_transform = Transform::from_xyz(
                (chunk_coord.x * CHUNK_SIZE) as f32,
                (chunk_coord.y * CHUNK_HEIGHT) as f32,
                (chunk_coord.z * CHUNK_SIZE) as f32
            );

            info!("Spawning chunks");

            let chunk = generate_single_chunk(&chunk_coord);

            let (indices, vertices) = render_indices_and_vertices(&chunk);

            let mesh_handle = mesh_manager.add(render_mesh(&indices, &vertices));

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HEIGHT{
                    for z in 0..CHUNK_SIZE {
                        if let Some(block) = chunk.get_block(&(x, y, z)) {
                            if !block.is_fully_surrounded && block.block_type != GameBlockType::Empty {
                                let block_transform = Transform::from_xyz(
                                    (chunk_coord.x * CHUNK_SIZE + x) as f32,
                                    (chunk_coord.y * CHUNK_HEIGHT + y) as f32,
                                    (chunk_coord.z * CHUNK_SIZE + z) as f32
                                );

                                commands.spawn((
                                    block_transform
                                ));
                            }
                        }
                    }
                }
            }

            let pbr = PbrBundle {
                transform: chunk_transform,
                mesh: mesh_handle,
                material: block_material.0.clone(),
                ..default()
            };

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

            commands.spawn((
                pbr,
                chunk,
                chunk_coord,
                RigidBody::Fixed,
                Collider::trimesh(
                    v,
                    i
                ),
                FreshEntity::default()
            ));
        }
    }

    world_generation_state.finished_generating = true;
}