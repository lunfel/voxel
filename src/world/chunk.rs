use std::mem::swap;
use std::ops::Deref;
use bevy::prelude::*;
use bevy::prelude::shape;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::{settings::CHUNK_SIZE, utils::point::Point3D};
use crate::systems::world_generation::{BlockMaterialHashMap, BlockMaterialMap};
use crate::world::block::{BlockCoord, GameBlockType};

use super::block::GameBlock;

#[derive(Deref, Clone, PartialEq, Eq, Hash)]
pub struct ChunkCoord(Point3D<usize>);

impl From<Point3D<usize>> for ChunkCoord {
    fn from(value: Point3D<usize>) -> Self {
        Self(value)
    }
}

impl From<(usize, usize, usize)> for ChunkCoord {
    fn from(value: (usize, usize, usize)) -> Self {
        Self(Point3D::from(value))
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct ChunkBlocks([[[GameBlock; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

impl ChunkBlocks {
    pub fn blocks_with_coord(&self) -> Vec<(Point3D<i8>, &GameBlock)> {
        let mut pairs: Vec<(Point3D<i8>, &GameBlock)> = Vec::with_capacity(CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    pairs.push(((x as i8, y as i8, z as i8).into(), &self.0[x][y][z]))
                }
            }
        }

        pairs
    }
}

pub struct GameChunk {
    chunk_coord: ChunkCoord,
    pub blocks: ChunkBlocks,
    block_entities: Vec<Entity>
}

type VertexBuffer = Vec<([f32; 3], [f32; 3], [f32; 2])>;

impl GameChunk {
    pub fn new(chunk_coord: ChunkCoord) -> Self {
        Self {
            chunk_coord,
            blocks: default(),
            block_entities: default()
        }
    }

    pub fn replace_block_entities(&mut self, mut new_entities: Vec<Entity>) -> Vec<Entity> {
        swap(&mut new_entities, &mut self.block_entities);

        // These are the old entities as they have been swapped
        new_entities
    }

    pub fn render_chunk(&self, mesh_manager: &mut ResMut<Assets<Mesh>>, block_material_mapping: &Res<BlockMaterialMap>) -> (Vec<PbrBundle>, u32) {
        self.render_naive(mesh_manager, block_material_mapping)
    }

    fn render_single_mesh() -> (PbrBundle, u32) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {}
            }
        }

        todo!()
    }

    fn render_naive(&self, mesh_manager: &mut ResMut<Assets<Mesh>>, block_material_mapping: &Res<BlockMaterialMap>) -> (Vec<PbrBundle>, u32) {
        let mut bundles = Vec::new();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut indices: Vec<u32> = vec![];
        let mut total_nb_faces = 0;
        let mut vertices: VertexBuffer = vec![];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_coord: BlockCoord = (x, y, z).into();
                    if let Some(block) = self.get_block(&block_coord) {
                        match block.block_type {
                            GameBlockType::Empty => (),
                            _ => {
                                create_custom_cube(&(x, y, z), &self, &mut mesh, &mut indices, &mut total_nb_faces, &mut vertices);

                                    // Collider::cuboid(0.5, 0.5, 0.5),
                                    // Friction {
                                    //     coefficient: 0.0,
                                    //     combine_rule: CoefficientCombineRule::Min
                                    // }));
                            }
                            _ => ()
                        }
                    }
                }
            }
        }

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        let indices = Indices::U32(indices);

        mesh.set_indices(Some(indices));

        let mesh_handle = mesh_manager.add(mesh);

        bundles.push(PbrBundle {
            mesh: mesh_handle.clone(),
            material: block_material_mapping.get(&GameBlockType::Ground).unwrap().clone(),
            transform: Transform::from_xyz(
                (self.chunk_coord.x * CHUNK_SIZE) as f32,
                (self.chunk_coord.y * CHUNK_SIZE) as f32,
                (self.chunk_coord.z * CHUNK_SIZE) as f32
            ),
            ..default()
        });

        (bundles, total_nb_faces * 2)
    }

    pub fn get_block<P>(&self, into_coord: &P) -> Option<&GameBlock>
    where P: Into<BlockCoord> + Clone
    {
        let coord: BlockCoord = (*into_coord).clone().into();

        self.blocks.get(coord.x)
            .and_then(|blocks_y| blocks_y.get(coord.y)
                .and_then(|blocks_z| blocks_z.get(coord.z))
            )
    }
}

fn create_custom_cube<P>(into_coord: &P, chunk: &GameChunk, mesh: &mut Mesh, indices: &mut Vec<u32>, total_nb_faces: &mut u32, vertices: &mut VertexBuffer)
    where P: Into<BlockCoord> + Clone
{
    let coord: BlockCoord = (*into_coord).clone().into();
    // suppose Y-up right hand, and camera look from +z to -z
    let sp = shape::Box::new(1.0, 1.0, 1.0);

    // let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
    // let mut indices: Vec<u32> = vec![];

    let indices_template = [0, 1, 2, 2, 3, 0];
    // let mut nb_faces: u32 = 0;

    let faces: [_; 6] = [
        (
            coord.front_neighbor(),
            [
                ([sp.min_x, sp.min_y, sp.max_z], [0., 0., 1.0], [0., 0.]),
                ([sp.max_x, sp.min_y, sp.max_z], [0., 0., 1.0], [1.0, 0.]),
                ([sp.max_x, sp.max_y, sp.max_z], [0., 0., 1.0], [1.0, 1.0]),
                ([sp.min_x, sp.max_y, sp.max_z], [0., 0., 1.0], [0., 1.0])
            ]
        ),
        (
            coord.back_neighbor(),
            [
                ([sp.min_x, sp.max_y, sp.min_z], [0., 0., -1.0], [1.0, 0.]),
                ([sp.max_x, sp.max_y, sp.min_z], [0., 0., -1.0], [0., 0.]),
                ([sp.max_x, sp.min_y, sp.min_z], [0., 0., -1.0], [0., 1.0]),
                ([sp.min_x, sp.min_y, sp.min_z], [0., 0., -1.0], [1.0, 1.0])
            ]
        ),
        (
            coord.right_neighbor(),
            [
                ([sp.max_x, sp.min_y, sp.min_z], [1.0, 0., 0.], [0., 0.]),
                ([sp.max_x, sp.max_y, sp.min_z], [1.0, 0., 0.], [1.0, 0.]),
                ([sp.max_x, sp.max_y, sp.max_z], [1.0, 0., 0.], [1.0, 1.0]),
                ([sp.max_x, sp.min_y, sp.max_z], [1.0, 0., 0.], [0., 1.0])
            ]
        ),
        (
            coord.left_neighbor(),
            [
                ([sp.min_x, sp.min_y, sp.max_z], [-1.0, 0., 0.], [1.0, 0.]),
                ([sp.min_x, sp.max_y, sp.max_z], [-1.0, 0., 0.], [0., 0.]),
                ([sp.min_x, sp.max_y, sp.min_z], [-1.0, 0., 0.], [0., 1.0]),
                ([sp.min_x, sp.min_y, sp.min_z], [-1.0, 0., 0.], [1.0, 1.0])
            ]
        ),
        (
            coord.top_neighbor(),
            [

                ([sp.max_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [1.0, 0.]),
                ([sp.min_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [0., 0.]),
                ([sp.min_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [0., 1.0]),
                ([sp.max_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [1.0, 1.0])
            ]
        ),
        (
            coord.bottom_neighbor(),
            [
                ([sp.max_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [0., 0.]),
                ([sp.min_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [1.0, 0.]),
                ([sp.min_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [1.0, 1.0]),
                ([sp.max_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [0., 1.0])
            ]
        )
    ];

    let block_offset: [f32; 3] = [
        coord.x as f32,
        coord.y as f32,
        coord.z as f32
    ];

    for (coord, attributes) in faces.iter() {
        let mut should_render_face = true;
        if let Some(coord) = coord {
            if let Some(cmp_block) = chunk.get_block(coord) {
                if cmp_block.block_type != GameBlockType::Empty {
                    should_render_face = false;
                }
            }
        }

        if should_render_face {
            attributes.iter()
                .for_each(|(position, normals, uv)| {
                    let v: [f32; 3] = position.iter()
                        .zip(block_offset)
                        .into_iter()
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

