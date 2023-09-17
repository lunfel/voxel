use std::mem::swap;
use std::ops::Deref;
use bevy::prelude::*;
use bevy::prelude::shape;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::{settings::CHUNK_SIZE, utils::point::Point3D};
use crate::systems::world_generation::{BlockMaterialHashMap, BlockMaterialMap};
use crate::world::block::GameBlockType;

use super::block::GameBlock;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ChunkCoord(Point3D<usize>);

impl Deref for ChunkCoord {
    type Target = Point3D<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

    pub fn render_chunk(&self, mesh_manager: &mut ResMut<Assets<Mesh>>, block_material_mapping: &Res<BlockMaterialMap>) -> Vec<PbrBundle> {
        self.render_naive(mesh_manager, block_material_mapping)

        // for (coord, block) in self.blocks.blocks_with_coord() {
        //     if let Some(block) = self.get_block(&coord) {
        //         if (block.block_type != GameBlockType::Empty) {
        //             let start_block_type = block.block_type;
        //
        //             let mut end_block_y: Option<Point3D<i8>> = None;
        //             let mut end_block_z: Option<Point3D<i8>> = None;
        //
        //             let mut next_x = coord;
        //
        //             loop {
        //                 next_x = next_x.right_neighbor();
        //                 let maybe_block_x = self.get_block(&next_x);
        //
        //                 if let Some(block_x) = maybe_block_x {
        //                     if block_x.block_type == start_block_type {
        //                         // Don't break
        //                     } else {
        //                         break;
        //                     }
        //                 } else {
        //                     break;
        //                 }
        //             }
        //
        //             let end_block_x = next_x;
        //
        //             loop {
        //                 // check that each lines of blocks are the same type between start_x and end_x for each y
        //             }
        //
        //             loop {
        //                 // check that each plane of block are the same type between start_x, start_y, end_x and end_y
        //             }
        //
        //
        //
        //
        //
        //             break;
        //         }
        //     } else {
        //
        //     }
        // }
        //
        // let mesh =  todo!();
        //
        // RenderedGameChunk::new(self, mesh)
    }

    fn render_naive(&self, mesh_manager: &mut ResMut<Assets<Mesh>>, block_material_mapping: &Res<BlockMaterialMap>) -> Vec<PbrBundle> {
        let mut bundles = Vec::new();

        info!("Inserting cubes in the world");

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_coord: Point3D<usize> = (x, y, z).into();
                    if let Some(block) = self.get_block(&block_coord) {
                        match block.block_type {
                            GameBlockType::Empty => (),
                            _ => {
                                if let Some(mesh) = create_custom_cube(&(x as i8, y as i8, z as i8), &self) {
                                    let mesh_handle = mesh_manager.add(mesh);
                                    bundles.push(PbrBundle {
                                        mesh: mesh_handle.clone(),
                                        material: block_material_mapping.get(&block.block_type).unwrap().clone(),
                                        transform: Transform::from_xyz(
                                            x as f32 + self.chunk_coord.x as f32 * CHUNK_SIZE as f32,
                                            y as f32 + self.chunk_coord.y as f32 * CHUNK_SIZE as f32,
                                            z as f32 + self.chunk_coord.z as f32 * CHUNK_SIZE as f32
                                        ),
                                        ..default()
                                    });
                                    // Collider::cuboid(0.5, 0.5, 0.5),
                                    // Friction {
                                    //     coefficient: 0.0,
                                    //     combine_rule: CoefficientCombineRule::Min
                                    // }));
                                }
                            }
                            _ => ()
                        }
                    }
                }
            }
        }

        bundles
    }

    pub fn get_block<P>(&self, maybe_into_coord: &P) -> Option<&GameBlock>
    where P: TryInto<Point3D<usize>> + Clone
    {
        let res: Result<Point3D<usize>, _> = (*maybe_into_coord).clone().try_into();

        if let Ok(coord) = res {
            self.blocks.get(coord.x)
                .and_then(|blocks_y| blocks_y.get(coord.y)
                    .and_then(|blocks_z| blocks_z.get(coord.z))
                )
        } else {
            None
        }
    }
}

fn create_custom_cube<P>(into_coord: &P, chunk: &GameChunk) -> Option<Mesh>
    where P: Into<Point3D<i8>> + Clone
{
    let coord: Point3D<i8> = (*into_coord).clone().into();
    // suppose Y-up right hand, and camera look from +z to -z
    let sp = shape::Box::new(1.0, 1.0, 1.0);

    let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
    let mut indices: Vec<u32> = vec![];

    let indices_template = [0, 1, 2, 2, 3, 0];
    let mut nb_faces = 0;

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

    for (coord, attributes) in faces.iter() {
        if let Some(cmp_block) = chunk.get_block(coord) {
            if cmp_block.block_type == GameBlockType::Empty {
                attributes.iter()
                    .for_each(|attribute| {
                        vertices.push(*attribute)
                    });

                indices_template.iter()
                    .map(|i| i + nb_faces * 4)
                    .for_each(|i| indices.push(i));

                nb_faces += 1;
            }
        } else {
            attributes.iter()
                .for_each(|attribute| {
                    vertices.push(*attribute)
                });

            indices_template.iter()
                .map(|i| i + nb_faces * 4)
                .for_each(|i| indices.push(i));

            nb_faces += 1;
        }
    }

    if nb_faces == 0 {
        return None
    }

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    let indices = Indices::U32(indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));
    Some(mesh)
}

