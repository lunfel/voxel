use bevy::prelude::*;
use bevy::render::mesh::Indices;
use crate::chunk::block::{VoxelBlock, VoxelBlockType};
use crate::game_world::coord::{LocalVoxelBlockCoord, LocalVoxelBlockOffset};
use crate::settings::{CHUNK_HEIGHT, CHUNK_SIZE};

pub type Vertex = [f32; 3];
type Normal = [f32; 3];
pub type UV = [f32; 2];
pub type VertexBuffer = Vec<(Vertex, Normal, UV)>;

impl Default for VoxelChunk {
    fn default() -> Self {
        Self([VoxelBlock::default(); (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as usize])
    }
}

#[derive(Component, Debug, Clone)]
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

        let faces: [_; 6] = [
            (
                coord + [0, 0, 1],
                [
                    ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([0., 0.], uv_offset)),
                    ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([1.0, 1.0], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([0., 1.0], uv_offset))
                ]
            ),
            (
                coord + [0, 0, -1],
                [
                    ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([0., 0.], uv_offset)),
                    ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([0., 1.0], uv_offset)),
                    ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([1.0, 1.0], uv_offset))
                ]
            ),
            (
                coord + [1, 0, 0],
                [
                    ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([0., 0.], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([1.0, 1.0], uv_offset)),
                    ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([0., 1.0], uv_offset))
                ]
            ),
            (
                coord + [-1, 0, 0],
                [
                    ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([0., 0.], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([0., 1.0], uv_offset)),
                    ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([1.0, 1.0], uv_offset))
                ]
            ),
            (
                coord + [0, 1, 0],
                [
                    ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([0., 0.], uv_offset)),
                    ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([0., 1.0], uv_offset)),
                    ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([1.0, 1.0], uv_offset))
                ]
            ),
            (
                coord + [0, -1, 0],
                [
                    ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([0., 0.], uv_offset)),
                    ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                    ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([1.0, 1.0], uv_offset)),
                    ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([0., 1.0], uv_offset))
                ]
            )
        ];

        let block_coord: LocalVoxelBlockCoord = LocalVoxelBlockCoord::from(*coord);

        let block_offset: [f32; 3] = [
            block_coord.x as f32,
            block_coord.y as f32,
            block_coord.z as f32
        ];

        for (coord, attributes) in faces.iter() {
            if let Some(cmp_block) = self.get_block(coord) {
                if cmp_block.block_type == VoxelBlockType::Empty {
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
}

fn apply_uv_offset(uv: UV, offset: f32) -> UV {
    let [u, v] = uv;

    [u / 4.0 + offset, v]
}

fn all_block_coords_iter() -> impl Iterator<Item = LocalVoxelBlockCoord> {
    (0..CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE).map(|index| LocalVoxelBlockCoord::from(LocalVoxelBlockOffset(index as usize)))
}

fn all_block_offsets_iter() -> impl Iterator<Item = LocalVoxelBlockOffset> {
    (0..CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE).map(|index| LocalVoxelBlockOffset(index as usize))
}