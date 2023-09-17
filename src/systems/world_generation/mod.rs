use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use noise::{NoiseFn, Perlin};

use crate::{settings::CHUNK_SIZE, world::{block::GameBlockType, chunk::GameChunk, GameWorld}};
use crate::world::chunk::ChunkCoord;

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

pub fn generate_single_chunk<'a, P>(coord: &P) -> GameChunk
where P: Into<ChunkCoord> + Clone
{
    let height_perlin = Perlin::new(1);
    let ground_layer_perlin = Perlin::new(2);
    let coord: ChunkCoord = (*coord).clone().into();

    let mut game_chunk = GameChunk::new(coord.clone());

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
    mut game_world: ResMut<GameWorld>,
    mut world_generation_state: ResMut<WorldGenerationState>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
    block_material_map: Res<BlockMaterialMap>,
    mut commands: Commands
) {
    info!("Generate world chunks");

    let mut total_triangles = 0;

    for x in 0..8 {
        for z in 0..8 {
            let point: ChunkCoord = (x as usize, 0 as usize, z as usize).into();
            let mut chunk = generate_single_chunk(&point);

            let (pbrs, nb_triangles) = chunk.render_chunk(&mut mesh_manager, &block_material_map);

            total_triangles += nb_triangles;

            let entities: Vec<_> = pbrs.into_iter()
                .map(|pbr| {
                    commands.spawn(pbr).id()
                })
                .collect();

            chunk.replace_block_entities(entities)
                .into_iter()
                .for_each(|_old_entity| {
                    // todo: remove old entities
                });

            game_world.chunks.insert(
                point,
                chunk
            );
        }
    }

    info!("Total Rendered triangles: {}", total_triangles);

    world_generation_state.finished_generating = true;
}