use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy_rapier3d::math::Vect;
use bevy_rapier3d::prelude::*;
use crate::chunk::block::{BlockMaterial, VoxelBlock, VoxelBlockType};
use crate::game_world::coord::{ChunkCoord, LocalVoxelBlockCoord, LocalVoxelBlockOffset};
use crate::game_world::GameWorld;
use crate::settings::{CHUNK_HEIGHT, CHUNK_SIZE, MAX_OFFSET};
use crate::utils::{VertexBuffer, UV};

impl Default for VoxelChunk {
    fn default() -> Self {
        Self([VoxelBlock::default(); (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as usize])
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct VoxelChunk(pub [VoxelBlock; (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as usize]);

impl VoxelChunk {
    pub fn update_block<P, F>(&mut self, into_coord: &P, update: F)
    where P: Into<LocalVoxelBlockOffset> + Clone,
          F: Fn(&mut VoxelBlock)
    {
        if let Some(block) = self.get_block_mut(into_coord) {
            update(block);
        }
    }

    pub fn get_block<P>(&self, into_coord: &P) -> Option<&VoxelBlock>
    where P: Into<LocalVoxelBlockOffset> + Clone
    {
        let coord = into_coord.clone().into();

        self.0.get(*coord)
    }

    pub fn get_block_mut<P>(&mut self, into_coord: &P) -> Option<&mut VoxelBlock>
    where P: Into<LocalVoxelBlockOffset> + Clone
    {
        let coord = into_coord.clone().into();

        self.0.get_mut(*coord)
    }

    pub fn render_indices_and_vertices(&self) -> (Indices, VertexBuffer) {
        let mut indices: Vec<u32> = vec![];
        let mut total_nb_faces: u32 = 0;
        let mut vertices: VertexBuffer = vec![];

        for coord in  all_block_offsets_iter() {
            if let Some(block) = self.get_block(&coord) {
                match block.block_type {
                    VoxelBlockType::Empty => (),
                    _ => self.render_chunk_block(&coord, &mut indices, &mut total_nb_faces, &mut vertices)
                }
            }
        }

        let indices = Indices::U32(indices);

        (indices, vertices)
    }

    fn render_chunk_block(&self, coord: &LocalVoxelBlockOffset, indices: &mut Vec<u32>, total_nb_faces: &mut u32, vertices: &mut VertexBuffer) {
        // suppose Y-up right hand, and camera look from +z to -z
        let sp = Cuboid::new(1.0, 1.0, 1.0);
        // let sp = shape::Box::new(1.0, 1.0, 1.0);

        let indices_template = [0, 1, 2, 2, 3, 0];

        let block = self.get_block(coord).expect("A block was expected here, but no block found");

        let uv_offset = match block.block_type {
            VoxelBlockType::Rock => 0.25,
            VoxelBlockType::Grass => 0.00,
            VoxelBlockType::Gem => 0.50,
            VoxelBlockType::Dirt => 0.75,
            _ => 0.00
        };

        // println!("Current: {:?}", coord);
        // println!("Front: {:?}", coord + [0, 0, 1]);
        // println!("Back: {:?}", coord + [0, 0, -1]);
        // println!("Right: {:?}", coord + [1, 0, 0]);
        // println!("Left: {:?}", coord + [-1, 0, 0]);
        // println!("Top: {:?}", coord + [0, 1, 0]);
        // println!("Bottom: {:?}", coord + [0, -1, 0]);

        let block_coord: LocalVoxelBlockCoord = LocalVoxelBlockCoord::from(*coord);


        let faces: [_; 6] = [
            (
                block_coord + [0, 0, 1],
                [
                    ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([0., 0.], uv_offset)),
                    ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([1.0, 1.0], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([0., 1.0], uv_offset))
                ]
            ),
            (
                block_coord + [0, 0, -1],
                [
                    ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([0., 0.], uv_offset)),
                    ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([0., 1.0], uv_offset)),
                    ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([1.0, 1.0], uv_offset))
                ]
            ),
            (
                block_coord + [1, 0, 0],
                [
                    ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([0., 0.], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([1.0, 1.0], uv_offset)),
                    ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([0., 1.0], uv_offset))
                ]
            ),
            (
                block_coord + [-1, 0, 0],
                [
                    ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([0., 0.], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([0., 1.0], uv_offset)),
                    ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([1.0, 1.0], uv_offset))
                ]
            ),
            (
                block_coord + [0, 1, 0],
                [
                    ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([0., 0.], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([0., 1.0], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([1.0, 1.0], uv_offset))
                ]
            ),
            (
                block_coord + [0, -1, 0],
                [
                    ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([0., 0.], uv_offset)),
                    ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([1.0, 1.0], uv_offset)),
                    ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([0., 1.0], uv_offset))
                ]
            )
        ];

        let block_offset: [f32; 3] = [
            block_coord.x as f32,
            block_coord.y as f32,
            block_coord.z as f32
        ];

        for (coord, attributes) in faces.iter() {
            let mut should_render_face = true;
            if let Some(coord) = coord {
                if coord.is_valid_chunk_voxel_coord() {
                    if let Some(cmp_block) = self.get_block(coord) {
                        if cmp_block.block_type != VoxelBlockType::Empty {
                            should_render_face = false;
                        }
                    }
                }
            }

            if should_render_face {
                attributes.iter()
                    .for_each(|(position, normals, uv)| {
                        let v: [f32; 3] = position.iter()
                            .zip(block_offset)
                            .map(|(v, offset)| v + offset)
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap();

                        vertices.push((
                            v,
                            *normals,
                            *uv
                        ))
                    });

                indices_template.iter()
                    .map(|i| i + (*total_nb_faces) * 4)
                    .for_each(|i| indices.push(i));

                *total_nb_faces += 1;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub mesh: Mesh,
    pub vertex: Vec<Vect>,
    pub indices: Vec<[u32; 3]>,
    pub chunk: VoxelChunk
}

pub fn add_new_chunks_to_game_world(
    mut game_world: ResMut<GameWorld>,
    query: Query<(&ChunkCoord, Entity), Added<VoxelChunk>>
) {
    for (coord, entity) in query.iter() {
        game_world.insert(*coord, entity);
    }
}

pub fn spawn_chunk_from_data(chunk_data: ChunkData, chunk_coord: ChunkCoord, block_material: &Res<BlockMaterial>, mesh_manager: &mut Assets<Mesh>, commands: &mut Commands) {
    commands.spawn((
        Transform::from(chunk_coord),
        Mesh3d(mesh_manager.add(chunk_data.mesh)),
        MeshMaterial3d(block_material.0.clone()),
        chunk_data.chunk,
        chunk_coord,
        // todo: re-enabled collisions laterss
        // RigidBody::Fixed,
        // Collider::trimesh(
        //     chunk_data.vertex,
        //     chunk_data.indices
        // ),
        Visibility::Visible
    ));
}

fn apply_uv_offset(uv: UV, offset: f32) -> UV {
    let [u, v] = uv;

    [u / 4.0 + offset, v]
}

fn all_block_coords_iter() -> impl Iterator<Item = LocalVoxelBlockCoord> {
    (0..MAX_OFFSET).map(|index| LocalVoxelBlockCoord::from(LocalVoxelBlockOffset(index as usize)))
}

fn all_block_offsets_iter() -> impl Iterator<Item = LocalVoxelBlockOffset> {
    (0..MAX_OFFSET).map(|index| LocalVoxelBlockOffset(index as usize))
}