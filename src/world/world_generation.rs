use std::ops::{Add, Div, Mul, Sub};
use bevy::app::{App, Plugin, Startup};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::log::{info, info_span};
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Color, Commands, Deref, DerefMut, FromWorld, Mesh, Mesh3d, MeshMaterial3d, Res, ResMut, Resource, Transform, Visibility, World};
use bevy::render::mesh::Indices;
use bevy::utils::hashbrown::HashMap;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::math::Vect;
use noise::{NoiseFn, Perlin};
use crate::settings::{Settings, CHUNK_HEIGHT, CHUNK_SIZE};
use crate::utils::point::Point3D;
use crate::world::block::{BlockCoord, GameBlockType};
use crate::world::chunk::{chunk_coordinates_to_world_transform, render_indices_and_vertices, render_mesh, ChunkCoord, GameChunk};
use crate::world::game_world::{ChunkKeepAlive, PendingAdditionToGameWorld};

#[derive(Resource, Default)]
pub struct WorldGenerationState {
    pub finished_generating: bool
}

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        info!("WorldGenerationPlugin initializing");
        app
            .init_resource::<WorldGenerationState>()
            .add_systems(Startup, generate_world);
        info!("WorldGenerationPlugin loaded");
    }
}

pub fn generate_world(
    mut world_generation_state: ResMut<WorldGenerationState>,
    block_material: Res<BlockMaterial>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    settings: Res<Settings>
) {
    info!("Generate world chunks");

    // Let's assume player is at 0,0,0 for now
    let player_position: Point3D<i32> = Point3D::default();

    info!("Spawning chunks START");
    // I think there is a bug where player_position.x and player_position.z should be devided by
    // CHUNK_SIZE. But has no effect right now because player position is 0,0
    let generation_range_x = player_position.x - settings.world.world_dimension..player_position.x + settings.world.world_dimension;
    let generation_range_z = player_position.z - settings.world.world_dimension..player_position.z + settings.world.world_dimension;
    for x in generation_range_x.clone() {
        for z in generation_range_z.clone() {
            let chunk_coord = ChunkCoord::new(x, z);
            let chunk_data = generate_chunk(&chunk_coord);

            spawn_chunk_from_data(chunk_data, chunk_coord, &block_material, &mut mesh_manager, &mut commands);
        }
    }

    info!("Spawning chunks END");

    world_generation_state.finished_generating = true;
}

pub fn spawn_chunk_from_data(chunk_data: ChunkData, chunk_coord: ChunkCoord, block_material: &Res<BlockMaterial>, mesh_manager: &mut Assets<Mesh>, commands: &mut Commands) {
    // for t in chunk_data.block_transforms.into_iter() {
    //     commands.spawn(t);
    // }

    commands.spawn((
        chunk_coordinates_to_world_transform(&chunk_coord),
        Mesh3d(mesh_manager.add(chunk_data.mesh)),
        MeshMaterial3d(block_material.0.clone()),
        chunk_data.chunk,
        chunk_coord,
        RigidBody::Fixed,
        Collider::trimesh(
            chunk_data.vertex,
            chunk_data.indices
        ),
        PendingAdditionToGameWorld,
        ChunkKeepAlive::default(),
        Visibility::Hidden
    ));
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    block_transforms: Vec<Transform>,
    mesh: Mesh,
    vertex: Vec<Vect>,
    indices: Vec<[u32; 3]>,
    chunk: GameChunk
}

pub fn generate_chunk(chunk_coord: &ChunkCoord) -> ChunkData {
    let chunk_transform = chunk_coordinates_to_world_transform(chunk_coord);

    let chunk = generate_single_chunk(chunk_coord);

    let (indices, vertices) = render_indices_and_vertices(&chunk);

    // let mesh = Mesh3d(mesh_manager.add(render_mesh(&indices, &vertices)));
    let mesh = render_mesh(&indices, &vertices);

    let mut block_transforms = vec![];

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                if let Some(block) = chunk.get_block(&(x, y, z)) {
                    if !block.is_fully_surrounded && block.block_type != GameBlockType::Empty {
                        let block_transform = Transform::from_xyz(
                            (chunk_coord.x * CHUNK_SIZE + x) as f32,
                            (CHUNK_HEIGHT + y) as f32,
                            (chunk_coord.y * CHUNK_SIZE + z) as f32
                        );

                        block_transforms.push(block_transform);
                        //commands.spawn(block_transform);
                    }
                }
            }
        }
    }

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
        block_transforms,
        mesh,
        vertex: v,
        indices: i,
        chunk
    }
}

pub struct WorldGenerationPlugin;

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct PerlinCoord([f64; 2]);

impl Mul<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn mul(self, rhs: f64) -> Self::Output {
        [
            self[0] * rhs,
            self[1] * rhs
        ]
    }
}

impl Div<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn div(self, rhs: f64) -> Self::Output {
        [
            self[0] / rhs,
            self[1] / rhs
        ]
    }
}

impl Add<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn add(self, rhs: f64) -> Self::Output {
        [
            self[0] + rhs,
            self[1] + rhs
        ]
    }
}

impl Sub<f64> for PerlinCoord {
    type Output = [f64; 2];

    fn sub(self, rhs: f64) -> Self::Output {
        [
            self[0] - rhs,
            self[1] - rhs
        ]
    }
}

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct PerlinCoord3d([f64; 3]);

impl Mul<f64> for PerlinCoord3d {
    type Output = [f64; 3];

    fn mul(self, rhs: f64) -> Self::Output {
        [
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
        ]
    }
}

pub fn generate_single_chunk<P>(coord: &P) -> GameChunk
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
    let height_base_amplitude: f64 = 80.0;
    let height_base_frequency: f64 = 1.0 / 120.0;

    let height_perlin_new = Perlin::new(1);

    let ground_layer_perlin = Perlin::new(2);
    let coord: ChunkCoord = (*coord).clone().into();

    let mut game_chunk = GameChunk::new();

    for x in 0..(CHUNK_SIZE as usize) {
        for y in 0..(CHUNK_HEIGHT as usize) {
            for z in 0..(CHUNK_SIZE as usize) {
                let offset = 0.1153;
                let perlin_coord = PerlinCoord([
                    (x as f64  + offset) + (coord.x as f64 * CHUNK_SIZE as f64),
                    (z as f64  + offset) + (coord.y as f64 * CHUNK_SIZE as f64)
                ]);

                let perlin_coord3d = PerlinCoord3d([
                    (x as f64  + offset) + (coord.x as f64 * CHUNK_SIZE as f64),
                    (y as f64  + offset) * 5.0,
                    (z as f64  + offset) + (coord.y as f64 * CHUNK_SIZE as f64)
                ]);

                // perlin.get gives an f64 value between -1 and 1
                // let continentality_value = ((continentality_perlin.get(perlin_coord * continentality_frequency) + 1.0) / 2.0) * continentality_amplitude;
                // let height_value = ((height_perlin.get(perlin_coord * frequency1) + 1.0) / 2.0) * (amplitude1 * (continentality_value + 0.1));
                // let height_value2 = ((height_perlin2.get(perlin_coord * frequency2) + 1.0) / 2.0) * amplitude2 * (continentality_value + 0.1);
                // let height = (height_value + height_value2) as usize;

                // New version of height map
                let mut height = 0;
                for octave in 1..3 {
                    let frequency = height_lacunarity.powf(octave as f64) * height_base_frequency;
                    let amplitude = height_persistence.powf(octave as f64) * height_base_amplitude;
                    let octave_height = (((height_perlin_new.get(perlin_coord * frequency) + 1.0) / 2.0) * amplitude) as usize;

                    height += octave_height;
                }

                let block_value = (block_perlin.get(perlin_coord3d * (1.0 / 40.0)) + 1.0) / 2.0;

                // info!("perlin value {}", block_value);

                if height >= y {
                    game_chunk.blocks[x][y][z].block_type = match block_value {
                        0.40..0.41 => GameBlockType::Gem,
                        0.41..0.60 => GameBlockType::Rock,
                        // 0.60..0.68 => GameBlockType::Empty,
                        0.68..0.70 => GameBlockType::Dirt,
                        0.70..1.0 => GameBlockType::Grass,
                        _ => GameBlockType::Dirt,
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
        let asset_server = world.resource_mut::<AssetServer>();
        let handle_image = asset_server.load("atlas.png");

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let handle_material = materials.add(handle_image);

        Self(handle_material)
    }
}

impl FromWorld for BlockMaterialMap {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let mut material_map: BlockMaterialHashMap = HashMap::new();

        material_map.insert(GameBlockType::Rock, materials.add(Color::srgba(79.0 / 255.0, 87.0 / 255.0, 99.0 / 255.0, 1.0)));
        material_map.insert(GameBlockType::Grass, materials.add(Color::srgba(76.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0, 1.0)));

        Self(material_map)
    }
}